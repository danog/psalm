<?php

declare(strict_types=1);

namespace Psalm\Tools\IdConvert;

use PhpParser\Comment\Doc;
use PhpParser\Node;
use PhpParser\Node\Arg;
use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\Scalar\String_;
use PhpParser\Node\Stmt;
use Psalm\Codebase;
use Psalm\Internal\Interner;
use Psalm\Internal\MethodIdentifier;
use Psalm\NodeTypeProvider;
use Psalm\Plugin\EventHandler\AfterClassLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\AfterFileAnalysisInterface;
use Psalm\Plugin\EventHandler\AfterFunctionLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\AfterFileAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\AfterFunctionLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use Psalm\Storage\ClassLikeStorage;
use Psalm\Storage\FunctionLikeParameter;
use Psalm\Storage\FunctionLikeStorage;
use Psalm\Storage\MethodStorage;
use Psalm\Type\Atomic\TNamedObject;
use Psalm\Type\Union;
use SimpleXMLElement;
use SplObjectStorage;
use Throwable;

require_once __DIR__ . '/ConvertPlugin.php';

/** Declarations of each file, its top-level code, and the default values of its declarations. */
final class FilePlugin implements AfterFileAnalysisInterface
{
    public static function afterAnalyzeFile(AfterFileAnalysisEvent $event): void
    {
        $file = $event->getStatementsSource()->getFilePath();
        if (!ConvertPlugin::inProject($file)) {
            return;
        }
        try {
            $src = $event->getStatementsSource();
            $codebase = $event->getCodebase();
            $decls = new Decls($codebase, $file);
            $decls->file($event->getStmts());
            foreach ($decls->defaults as [$slot, $expr, $self]) {
                (new Walker($codebase, $file, null))->defaultValue($slot, $expr, $self);
            }
            $types = $src instanceof \Psalm\Internal\Analyzer\FileAnalyzer ? $src->getNodeTypeProvider() : null;
            $w = new Walker($codebase, $file, $types);
            $w->setContext(null, '', $src->getAliases());
            $w->file($event->getStmts());
        } catch (Throwable $e) {
            ConvertPlugin::out(['kind' => 'error', 'msg' => 'file: ' . $e->getMessage() . ' @' . $e->getFile() . ':' . $e->getLine(), 'file' => $file]);
        }
    }
}

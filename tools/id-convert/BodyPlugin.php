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

/** Facts of each function-like body (with its own node types). */
final class BodyPlugin implements AfterFunctionLikeAnalysisInterface
{
    public static function afterStatementAnalysis(AfterFunctionLikeAnalysisEvent $event): ?bool
    {
        $stmt = $event->getStmt();
        // a trait's method is analyzed from each using class: its nodes are the trait file's
        $file = $event->getFunctionlikeStorage()->location?->file_path ?? $event->getStatementsSource()->getFilePath();
        if (!ConvertPlugin::inProject($file)) {
            return null;
        }
        try {
            $src = $event->getStatementsSource();
            $w = new Walker($event->getCodebase(), $file, $event->getNodeTypeProvider());
            $w->setContext($src->getFQCLN(), $src->getNamespace() ?? '', $src->getAliases());
            $w->functionLike($stmt, $event->getFunctionlikeStorage());
        } catch (Throwable $e) {
            ConvertPlugin::out(['kind' => 'error', 'msg' => 'fn: ' . $e->getMessage() . ' @' . $e->getFile() . ':' . $e->getLine(), 'file' => $file]);
        }
        return null;
    }
}

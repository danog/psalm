<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use PhpParser\Node;
use PhpParser\Node\Arg;
use PhpParser\Node\Expr;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\NullableType;
use PhpParser\Node\Scalar\String_;
use PhpParser\Node\Stmt;
use Psalm\Codebase;
use Psalm\Internal\Interner;
use Psalm\Internal\MethodIdentifier;
use Psalm\Internal\Sym;
use Psalm\NodeTypeProvider;
use Psalm\Plugin\EventHandler\AfterClassLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\AfterFunctionLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\AfterFunctionLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use Psalm\Storage\ClassLikeStorage;
use Psalm\Storage\MethodStorage;
use Psalm\Type\Atomic\TNamedObject;
use ReflectionClass;
use SimpleXMLElement;
use SplObjectStorage;
use Throwable;

/**
 * Facts for migrating string names to interned ids over a flow graph of string slots (migrate.php solves and
 * edits). Slots: `P:method|param` (a plain `string` parameter), `F:class|prop` (a `string`-typed property),
 * `R:method` (a `string` return), `L:method|var` (a local holding strings).
 *
 *  - `decl`: a slot's declaration ranges (type node, docblock type) and whether its signature can change;
 *  - `flow`: a value flowing into a slot (argument, assignment, return), its source classified: another slot's read
 *    (`slot`), an expression with an id form (`edit`: TNamedObject->value => ->name, storage->name => ->id,
 *    MethodIdentifier->fq_class_name => ->class_id, literal / X::class => Sym), or any other string (`str`);
 *  - `use`: a slot read that is not a flow: inside `Interner::intern(<read>)` (`intern`, the call's range), or
 *    anything else (`other`: it will read `Interner::lookup(<id>)`).
 */
final class IdMigratePlugin implements PluginEntryPointInterface, AfterClassLikeAnalysisInterface
{
    /** @var ?array<int, string> */
    private static ?array $sym = null;

    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function root(): string
    {
        return dirname(__DIR__, 2);
    }

    public static function inSrc(string $file): bool
    {
        return str_starts_with($file, self::root() . '/src/');
    }

    public static function isPlainString(?Node $t): bool
    {
        return $t instanceof Identifier && strtolower($t->name) === 'string';
    }

    // ---------------------------------------------------------------- declarations: properties
    public static function afterStatementAnalysis(AfterClassLikeAnalysisEvent $event): ?bool
    {
        $stmt = $event->getStmt();
        $storage = $event->getClasslikeStorage();
        $file = $event->getStatementsSource()->getFilePath();
        if (!self::inSrc($file)) {
            return null;
        }
        try {
            foreach ($stmt->getProperties() as $prop) {
                if (!self::isPlainString($prop->type) || count($prop->props) !== 1) {
                    continue;
                }
                $pp = $prop->props[0];
                $name = $pp->name->name;
                $default = $pp->default;
                $doc = self::docType($prop->getDocComment(), '@var', null);
                self::out(['kind' => 'decl', 'slot' => 'F:' . strtolower(\Psalm\Internal\Interner::lookup($storage->id)) . '|' . $name, 'file' => $file,
                    'type' => [$prop->type->getStartFilePos(), $prop->type->getEndFilePos() + 1],
                    'doc' => $doc, 'fixed' => $default === null, 'why' => $default === null ? null : 'default'
                    , 'readonly' => $prop->isReadonly()]);
            }
            // promoted constructor parameters are properties too: not migrated (the property and the parameter
            // would have to move together)
        } catch (Throwable $e) {
            self::out(['kind' => 'error', 'msg' => 'class: ' . $e->getMessage()]);
        }
        return null;
    }

    /** @return ?array{int, int} range of the type in the docblock */
    public static function docType(?\PhpParser\Comment\Doc $doc, string $tag, ?string $var): ?array
    {
        if ($doc === null) {
            return null;
        }
        $rx = $var === null
            ? '/' . preg_quote($tag, '/') . '\s+(\S+)/'
            : '/' . preg_quote($tag, '/') . '\s+(\S+)\s+\$' . preg_quote($var, '/') . '\b/';
        if (!preg_match($rx, $doc->getText(), $m, PREG_OFFSET_CAPTURE)) {
            return null;
        }
        $base = $doc->getStartFilePos();
        return [$base + $m[1][1], $base + $m[1][1] + strlen($m[1][0])];
    }

    /** @param array<string, mixed> $row */
    public static function out(array $row): void
    {
        $path = getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/id-migrate.jsonl';
        file_put_contents($path, json_encode($row, JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
    }
}

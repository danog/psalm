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

require_once __DIR__ . '/Types.php';
require_once __DIR__ . '/TypeStr.php';
require_once __DIR__ . '/Decls.php';
require_once __DIR__ . '/Walker.php';

/**
 * Shared by FilePlugin / BodyPlugin (Psalm registers a plugin file's class itself as the hook handler).
 *
 * Facts for the single-pass string -> id conversion (convert.php solves and edits). Run by any Psalm (the
 * fork's) over a project whose root is env ID_CONVERT_ROOT; facts go to env ID_CONVERT_OUT (JSON lines).
 *
 * Slots (string storage locations): F:<class>::$<prop>, P:<fn>|<param>, R:<fn>, L:<fn>|<var>, E:<file>:<pos>
 * (synthetic: the result of an array builtin); a path suffix (#k / #v, nested) addresses array keys / values.
 * <fn> is <class>::<method> (lowercase), a function name, or closure@<file>:<pos>.
 *
 * Facts: slot (candidate paths, declaration ranges, seed/stop), tie (convert together), stop, j (a junction: an
 * expression range whose value, from `src`, is consumed by `dst`), cmp (two operands compared), arruse (an array
 * whose elements a string consumer reads), unres (a member accessed through an unknown receiver).
 */
final class ConvertPlugin
{
    public static function root(): string
    {
        return rtrim((string) getenv('ID_CONVERT_ROOT'), '/') . '/';
    }

    /** Read-only callers: code that uses the project's API but is not converted (its slots stay strings). */
    public const READ_ONLY = ['vendor/psalm/plugin-phpunit/', 'vendor/psalm/plugin-mockery/'];

    public static function inProject(string $file): bool
    {
        $r = self::root();
        if (!str_starts_with($file, $r)) {
            return false;
        }
        return !str_starts_with($file, $r . 'vendor/') || self::readOnly($file);
    }

    public static function readOnly(string $file): bool
    {
        foreach (self::READ_ONLY as $dir) {
            if (str_starts_with($file, self::root() . $dir)) {
                return true;
            }
        }
        return false;
    }

    /** @param array<string, mixed> $row */
    public static function out(array $row): void
    {
        file_put_contents((string) getenv('ID_CONVERT_OUT'),
            json_encode($row, JSON_UNESCAPED_SLASHES | JSON_INVALID_UTF8_SUBSTITUTE) . "\n", FILE_APPEND | LOCK_EX);
    }
}

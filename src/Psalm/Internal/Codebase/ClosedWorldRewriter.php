<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use Override;
use PhpParser;
use PhpParser\NodeVisitorAbstract;

use function array_map;

/**
 * Applies the simplifications of the whole-program analysis to a freshly parsed file (see
 * ClosedWorld::simplifyProgram): dead branches lose their statements, resolved dynamic constructs become static.
 *
 * @internal
 */
final class ClosedWorldRewriter extends NodeVisitorAbstract
{
    private int $extracts = 0;

    /**
     * @param array<int, true> $dead
     * @param array<int, array{string, mixed}> $rewrites
     */
    public function __construct(
        private readonly string $file,
        private readonly array $dead,
        private readonly array $rewrites,
    ) {
    }

    /**
     * A branch that only guards (ends in return, throw or exit, or reports an error): it is kept as written even
     * when proven dead, so that a wrong proof cannot remove a check.
     *
     * @param array<PhpParser\Node\Stmt> $stmts
     */
    private static function isGuard(array $stmts): bool
    {
        $last = $stmts[\count($stmts) - 1] ?? null;
        if ($last instanceof PhpParser\Node\Stmt\Return_
            || ($last instanceof PhpParser\Node\Stmt\Expression
                && ($last->expr instanceof PhpParser\Node\Expr\Throw_ || $last->expr instanceof PhpParser\Node\Expr\Exit_))
        ) {
            return true;
        }
        $finder = new PhpParser\NodeFinder();
        return $finder->findFirst($stmts, static fn(PhpParser\Node $n): bool => $n instanceof PhpParser\Node\Expr\FuncCall
            && $n->name instanceof PhpParser\Node\Name
            && \in_array(\strtolower($n->name->getLast()), ['trigger_error', 'user_error', 'error_log', 'exit', 'die'], true)) !== null;
    }

    #[Override]
    public function leaveNode(PhpParser\Node $node): PhpParser\Node|array|null
    {
        $pos = (int) $node->getAttribute('startFilePos');
        if (($node instanceof PhpParser\Node\Stmt\If_ || $node instanceof PhpParser\Node\Stmt\ElseIf_
                || $node instanceof PhpParser\Node\Stmt\Else_)
            && isset($this->dead[$pos])
            && $node->stmts !== []
            && !self::isGuard($node->stmts)
        ) {
            // fail closed: should the proof ever be wrong (an entry path outside the model), the branch traps
            // instead of silently doing nothing
            $line = $node->getStartLine();
            $at = $node->getAttributes();
            $node->stmts = [new PhpParser\Node\Stmt\Expression(new PhpParser\Node\Expr\Throw_(
                new PhpParser\Node\Expr\New_(
                    new PhpParser\Node\Name\FullyQualified('Error'),
                    [new PhpParser\Node\Arg(new PhpParser\Node\Scalar\String_(
                        'closed-world: branch proven dead at ' . $this->file . ':' . $line,
                        $at,
                    ), false, false, $at)],
                    $at,
                ),
                $at,
            ), $at)];
            return null;
        }
        $rewrite = $this->rewrites[$pos] ?? null;
        if ($rewrite === null) {
            return null;
        }
        [$kind, $data] = $rewrite;
        $attrs = $node->getAttributes();
        if ($kind === 'var' && $node instanceof PhpParser\Node\Expr\Variable) {
            /** @var string $data */
            return new PhpParser\Node\Expr\Variable($data, $attrs);
        }
        if ($kind === 'args' && $node instanceof PhpParser\Node\Expr\FuncCall) {
            /** @var list<string> $data */
            return new PhpParser\Node\Expr\Array_(
                array_map(
                    static fn(string $name): PhpParser\Node\ArrayItem => new PhpParser\Node\ArrayItem(
                        new PhpParser\Node\Expr\Variable($name, $attrs),
                        null,
                        false,
                        $attrs,
                    ),
                    $data,
                ),
                $attrs + ['kind' => PhpParser\Node\Expr\Array_::KIND_SHORT],
            );
        }
        if ($kind === 'extract' && $node instanceof PhpParser\Node\Stmt\Expression
            && $node->expr instanceof PhpParser\Node\Expr\FuncCall
        ) {
            /** @var list<string> $data */
            $tmp = '__extract' . ($this->extracts++) . '_' . $pos;
            $stmts = [new PhpParser\Node\Stmt\Expression(new PhpParser\Node\Expr\Assign(
                new PhpParser\Node\Expr\Variable($tmp, $attrs),
                $node->expr->getArgs()[0]->value,
                $attrs,
            ), $attrs)];
            foreach ($data as $key) {
                $stmts[] = new PhpParser\Node\Stmt\Expression(new PhpParser\Node\Expr\Assign(
                    new PhpParser\Node\Expr\Variable($key, $attrs),
                    new PhpParser\Node\Expr\ArrayDimFetch(
                        new PhpParser\Node\Expr\Variable($tmp, $attrs),
                        new PhpParser\Node\Scalar\String_($key, $attrs),
                        $attrs,
                    ),
                    $attrs,
                ), $attrs);
            }
            return $stmts;
        }
        return null;
    }
}

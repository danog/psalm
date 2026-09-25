<?php

/**
 * Check-run plugin (psalm-check.xml, loaded by the unconverted Psalm): reports an int operand where a string is
 * built (concatenation, interpolation, `(string)` cast). The type check cannot see an id that silently became part of
 * a string where its name used to be, so the report classifies it with everything else: the issues an unconverted
 * tree has too (real ints printed) are subtracted, what is left is an id that needs `Interner::lookup()`.
 */

declare(strict_types=1);

namespace IdConvert;

use PhpParser\Node\Expr;
use PhpParser\Node\Scalar\InterpolatedString;
use Psalm\CodeLocation;
use Psalm\Issue\PluginIssue;
use Psalm\IssueBuffer;
use Psalm\Plugin\EventHandler\AfterExpressionAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterExpressionAnalysisEvent;
use Psalm\Type\Atomic\TInt;

final class IdStringPlugin implements AfterExpressionAnalysisInterface
{
    /** @return list<Expr> */
    private static function leaves(Expr $e): array
    {
        return $e instanceof Expr\BinaryOp\Concat ? [...self::leaves($e->left), ...self::leaves($e->right)] : [$e];
    }

    public static function afterExpressionAnalysis(AfterExpressionAnalysisEvent $event): ?bool
    {
        $expr = $event->getExpr();
        $operands = match (true) {
            // Psalm walks a concatenation chain without an event per inner node: its leaves
            $expr instanceof Expr\BinaryOp\Concat => self::leaves($expr),
            $expr instanceof Expr\AssignOp\Concat => [$expr->expr],
            $expr instanceof Expr\Cast\String_ => [$expr->expr],
            $expr instanceof InterpolatedString => array_values(array_filter($expr->parts, static fn($p): bool => $p instanceof Expr)),
            default => [],
        };
        // where the operand sits: a double-quoted string part is split out of the string by the fixer
        $how = !$expr instanceof InterpolatedString ? 'built into'
            : ($expr->getAttribute('kind') === \PhpParser\Node\Scalar\String_::KIND_DOUBLE_QUOTED ? 'interpolated into' : 'heredoc-interpolated into');
        $source = $event->getStatementsSource();
        foreach ($operands as $operand) {
            $type = $source->getNodeTypeProvider()->getType($operand);
            if ($type === null) {
                continue;
            }
            $ints = 0;
            foreach ($type->getAtomicTypes() as $atomic) {
                if ($atomic::class === TInt::class) {
                    $ints++;
                } elseif (!$atomic instanceof \Psalm\Type\Atomic\TNull) {
                    continue 2;
                }
            }
            if ($ints === 0) {
                continue;
            }
            $contents = $event->getCodebase()->getFileContents($source->getFilePath());
            $start = (int) $operand->getAttribute('startFilePos');
            $text = substr($contents, $start, (int) $operand->getAttribute('endFilePos') + 1 - $start);
            IssueBuffer::maybeAdd(
                new IdString($type->getId() . ' ' . $text . ' ' . $how . ' a string', new CodeLocation($source, $operand)),
                $source->getSuppressedIssues(),
            );
        }
        return null;
    }
}

final class IdString extends PluginIssue
{
}

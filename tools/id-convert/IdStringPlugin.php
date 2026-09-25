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
    /**
     * An int-keyed map (an id-keyed map, after the conversion) through a function that renumbers int keys, or spread
     * into an array literal: its keys are lost.
     */
    private static function renumbered(AfterExpressionAnalysisEvent $event): void
    {
        $expr = $event->getExpr();
        $arrays = [];
        if ($expr instanceof Expr\FuncCall && $expr->name instanceof \PhpParser\Node\Name) {
            $fn = strtolower($expr->name->toString());
            $args = $expr->getArgs();
            if (in_array($fn, ['array_merge', 'array_merge_recursive', 'array_splice', 'array_shift', 'array_unshift'], true)
                || ($fn === 'array_slice' && !isset($args[3]))
            ) {
                $arrays = array_map(static fn($a) => $a->value, $fn === 'array_merge' || $fn === 'array_merge_recursive' ? $args : array_slice($args, 0, 1));
            }
        } elseif ($expr instanceof Expr\Array_) {
            foreach ($expr->items as $item) {
                if ($item !== null && $item->unpack) {
                    $arrays[] = $item->value;
                }
            }
        }
        $source = $event->getStatementsSource();
        foreach ($arrays as $array) {
            $type = $source->getNodeTypeProvider()->getType($array);
            if ($type === null) {
                continue;
            }
            foreach ($type->getAtomicTypes() as $atomic) {
                $key = null;
                if ($atomic instanceof \Psalm\Type\Atomic\TArray) {
                    $key = $atomic->type_params[0];
                } elseif ($atomic instanceof \Psalm\Type\Atomic\TKeyedArray && !$atomic->is_list && $atomic->fallback_params !== null) {
                    $key = $atomic->fallback_params[0];
                }
                if ($key === null) {
                    continue;
                }
                foreach ($key->getAtomicTypes() as $k) {
                    if ($k::class === TInt::class) {
                        $contents = $event->getCodebase()->getFileContents($source->getFilePath());
                        $start = (int) $array->getAttribute('startFilePos');
                        $text = substr($contents, $start, (int) $array->getAttribute('endFilePos') + 1 - $start);
                        IssueBuffer::maybeAdd(
                            new IdKeysRenumbered(
                                'int keys of ' . $text . ' renumbered by ' . ($expr instanceof Expr\Array_ ? 'a spread' : $fn),
                                new CodeLocation($source, $array),
                            ),
                            $source->getSuppressedIssues(),
                        );
                        continue 3;
                    }
                }
            }
        }
    }

    /**
     * An int offset (an id, after the conversion) probing a map keyed by strings, where Psalm itself is lenient:
     * `isset($map[$id])`, `array_key_exists($id, $map)`.
     */
    private static function idOffsets(AfterExpressionAnalysisEvent $event): void
    {
        $expr = $event->getExpr();
        $pairs = [];
        if ($expr instanceof Expr\Isset_) {
            foreach ($expr->vars as $var) {
                if ($var instanceof Expr\ArrayDimFetch && $var->dim !== null) {
                    $pairs[] = [$var->dim, $var->var];
                }
            }
        } elseif ($expr instanceof Expr\FuncCall && $expr->name instanceof \PhpParser\Node\Name
            && strtolower($expr->name->toString()) === 'array_key_exists' && count($expr->getArgs()) === 2
        ) {
            $pairs[] = [$expr->getArgs()[0]->value, $expr->getArgs()[1]->value];
        }
        $source = $event->getStatementsSource();
        $types = $source->getNodeTypeProvider();
        foreach ($pairs as [$offset, $array]) {
            $offset_type = $types->getType($offset);
            $array_type = $types->getType($array);
            if ($offset_type === null || $array_type === null || !$offset_type->isSingle()
                || $offset_type->getSingleAtomic()::class !== TInt::class
            ) {
                continue;
            }
            foreach ($array_type->getAtomicTypes() as $atomic) {
                if (!$atomic instanceof \Psalm\Type\Atomic\TArray) {
                    continue;
                }
                $keys = $atomic->type_params[0]->getAtomicTypes();
                $string_keys = array_filter($keys, static fn($k): bool => $k instanceof \Psalm\Type\Atomic\TString);
                if ($string_keys === [] || count($string_keys) !== count($keys)) {
                    continue;
                }
                $contents = $event->getCodebase()->getFileContents($source->getFilePath());
                $start = (int) $offset->getAttribute('startFilePos');
                $text = substr($contents, $start, (int) $offset->getAttribute('endFilePos') + 1 - $start);
                IssueBuffer::maybeAdd(
                    new IdOffsetIntoStringKeys(
                        'int ' . $text . ' probes a map keyed by ' . $atomic->type_params[0]->getId(),
                        new CodeLocation($source, $offset),
                    ),
                    $source->getSuppressedIssues(),
                );
                continue 2;
            }
        }
    }

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
        self::renumbered($event);
        self::idOffsets($event);
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

final class IdKeysRenumbered extends PluginIssue
{
}

final class IdOffsetIntoStringKeys extends PluginIssue
{
}

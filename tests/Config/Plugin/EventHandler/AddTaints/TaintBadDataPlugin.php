<?php

declare(strict_types=1);

namespace Psalm\Example\Plugin;

use Psalm\Internal\Interner;

use Override;
use PhpParser\Node\Expr\Variable;
use Psalm\Plugin\EventHandler\AddTaintsInterface;
use Psalm\Plugin\EventHandler\Event\AddRemoveTaintsEvent;
use Psalm\Type\TaintKind;

/**
 * Add input taints to all variables named 'bad_data'
 *
 * @psalm-suppress UnusedClass
 * @psalm-immutable
 */
final class TaintBadDataPlugin implements AddTaintsInterface
{
    /**
     * Called to see what taints should be added
     *
     * @psalm-mutation-free
     */
    #[Override]
    public static function addTaints(AddRemoveTaintsEvent $event): int
    {
        $expr = $event->getExpr();

        if (!$expr instanceof Variable) {
            return 0;
        }

        switch ($expr->name) {
            case Interner::intern('bad_data'):
                return TaintKind::ALL_INPUT;
            case Interner::intern('bad_sql'):
                return TaintKind::INPUT_SQL;
            case Interner::intern('bad_html'):
                return TaintKind::INPUT_HTML;
            case Interner::intern('bad_eval'):
                return TaintKind::INPUT_EVAL;
            case Interner::intern('bad_file'):
                return TaintKind::INPUT_FILE;
        }

        return 0;
    }
}

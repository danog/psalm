<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\AfterStatementAnalysisEvent;

/**
 * @api
 */
interface AfterStatementAnalysisInterface extends HookInterface
{
    /**
     * Called after a statement has been checked
     *
     * @return null|false
     */
    public static function afterStatementAnalysis(AfterStatementAnalysisEvent $event): ?bool;
}

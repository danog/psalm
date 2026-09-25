<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\AfterAnalysisEvent;

/**
 * @api
 */
interface AfterAnalysisInterface extends HookInterface
{
    /**
     * Called after analysis is complete
     */
    public static function afterAnalysis(AfterAnalysisEvent $event): void;
}

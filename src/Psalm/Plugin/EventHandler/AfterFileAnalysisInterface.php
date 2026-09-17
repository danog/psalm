<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\AfterFileAnalysisEvent;

/**
 * @api
 */
interface AfterFileAnalysisInterface extends HookInterface
{
    /**
     * Called after a file has been checked
     */
    public static function afterAnalyzeFile(AfterFileAnalysisEvent $event): void;
}

<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\BeforeFileAnalysisEvent;

/**
 * @api
 */
interface BeforeFileAnalysisInterface extends HookInterface
{
    /**
     * Called before a file has been checked
     */
    public static function beforeAnalyzeFile(BeforeFileAnalysisEvent $event): void;
}

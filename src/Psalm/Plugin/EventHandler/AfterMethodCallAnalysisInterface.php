<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\AfterMethodCallAnalysisEvent;

/**
 * @api
 */
interface AfterMethodCallAnalysisInterface extends HookInterface
{
    public static function afterMethodCallAnalysis(AfterMethodCallAnalysisEvent $event): void;
}

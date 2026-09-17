<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\AfterEveryFunctionCallAnalysisEvent;

/**
 * @api
 */
interface AfterEveryFunctionCallAnalysisInterface extends HookInterface
{
    public static function afterEveryFunctionCallAnalysis(AfterEveryFunctionCallAnalysisEvent $event): void;
}

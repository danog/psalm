<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\BeforeExpressionAnalysisEvent;

/**
 * @api
 */
interface BeforeExpressionAnalysisInterface extends HookInterface
{
    /**
     * Called before an expression is checked
     */
    public static function beforeExpressionAnalysis(BeforeExpressionAnalysisEvent $event): ?bool;
}

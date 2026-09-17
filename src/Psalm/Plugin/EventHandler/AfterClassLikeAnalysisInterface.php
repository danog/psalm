<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\AfterClassLikeAnalysisEvent;

/**
 * @api
 */
interface AfterClassLikeAnalysisInterface extends HookInterface
{
    /**
     * Called after a statement has been checked
     *
     * @return null|false
     * @phpcsSuppress SlevomatCodingStandard.TypeHints.ReturnTypeHint
     */
    public static function afterStatementAnalysis(AfterClassLikeAnalysisEvent $event);
}

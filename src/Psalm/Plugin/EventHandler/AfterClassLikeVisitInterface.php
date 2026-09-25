<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\AfterClassLikeVisitEvent;

/**
 * @api
 */
interface AfterClassLikeVisitInterface extends HookInterface
{
    /**
     * @return void
     * @phpcsSuppress SlevomatCodingStandard.TypeHints.ReturnTypeHint
     */
    public static function afterClassLikeVisit(AfterClassLikeVisitEvent $event);
}

<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\AfterClassLikeExistenceCheckEvent;

/**
 * @api
 */
interface AfterClassLikeExistenceCheckInterface extends HookInterface
{
    public static function afterClassLikeExistenceCheck(AfterClassLikeExistenceCheckEvent $event): void;
}

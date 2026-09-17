<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\PropertyVisibilityProviderEvent;

/**
 * @api
 */
interface PropertyVisibilityProviderInterface extends HookInterface
{
    /**
     * @return list<string>
     */
    public static function getClassLikeNames(): array;

    public static function isPropertyVisible(PropertyVisibilityProviderEvent $event): ?bool;
}

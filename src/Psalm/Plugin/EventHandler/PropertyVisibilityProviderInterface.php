<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\EventHandler\Event\PropertyVisibilityProviderEvent;

interface PropertyVisibilityProviderInterface
{
    /**
     * @return list<string>
     */
    public static function getClassLikeNames(): array;

    public static function isPropertyVisible(PropertyVisibilityProviderEvent $event): ?bool;
}

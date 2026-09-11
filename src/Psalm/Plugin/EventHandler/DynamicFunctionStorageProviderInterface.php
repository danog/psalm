<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\DynamicFunctionStorage;
use Psalm\Plugin\EventHandler\Event\DynamicFunctionStorageProviderEvent;

interface DynamicFunctionStorageProviderInterface
{
    /**
     * @return list<lowercase-string>
     */
    public static function getFunctionIds(): array;

    public static function getFunctionStorage(DynamicFunctionStorageProviderEvent $event): ?DynamicFunctionStorage;
}

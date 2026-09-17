<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\DynamicFunctionStorage;
use Psalm\Plugin\EventHandler\Event\DynamicFunctionStorageProviderEvent;

/**
 * @api
 */
interface DynamicFunctionStorageProviderInterface extends HookInterface
{
    /**
     * @return list<lowercase-string>
     */
    public static function getFunctionIds(): array;

    public static function getFunctionStorage(DynamicFunctionStorageProviderEvent $event): ?DynamicFunctionStorage;
}

<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\FunctionParamsProviderEvent;
use Psalm\Storage\FunctionLikeParameter;

/**
 * @api
 */
interface FunctionParamsProviderInterface extends HookInterface
{
    /**
     * @return list<lowercase-string>
     */
    public static function getFunctionIds(): array;

    /**
     * @return ?array<int, FunctionLikeParameter>
     */
    public static function getFunctionParams(FunctionParamsProviderEvent $event): ?array;
}

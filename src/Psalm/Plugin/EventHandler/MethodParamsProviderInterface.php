<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\MethodParamsProviderEvent;
use Psalm\Storage\FunctionLikeParameter;

/**
 * @api
 */
interface MethodParamsProviderInterface extends HookInterface
{
    /**
     * @return list<string>
     */
    public static function getClassLikeNames(): array;

    /**
     * @return ?array<int, FunctionLikeParameter>
     */
    public static function getMethodParams(MethodParamsProviderEvent $event): ?array;
}

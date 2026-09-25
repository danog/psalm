<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

/**
 * @api
 */
interface ClassFilePathProviderInterface extends HookInterface
{
    /**
     * @param class-string $class
     */
    public static function getClassFilePath(string $class): ?string;
}

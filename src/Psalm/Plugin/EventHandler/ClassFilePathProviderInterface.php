<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

/**
 * @api
 */
interface ClassFilePathProviderInterface
{
    /**
     * @param int $class
     */
    public static function getClassFilePath(int $class): ?string;
}

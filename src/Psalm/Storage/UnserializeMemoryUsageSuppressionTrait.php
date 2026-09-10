<?php

declare(strict_types=1);

namespace Psalm\Storage;

/**
 * Suppresses memory usage when unserializing objects.
 *
 * Workaround for the problem that objects retrieved with `\unserialize()`
 * build unnecessary dynamic property tables, resulting in larger memory
 * consumption.
 *
 * @see https://github.com/php/php-src/issues/10126
 * @psalm-immutable
 */
trait UnserializeMemoryUsageSuppressionTrait
{
    /**
     * @psalm-external-mutation-free
     */
    public function __unserialize(array $properties): void
    {
        // objects are never unserialized in the compiled program; property names cannot be looked up dynamically
        throw new \LogicException('Unserialization of ' . static::class . ' is not supported');
    }
}

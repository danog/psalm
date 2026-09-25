<?php

declare(strict_types=1);

namespace Psalm\Exception;

use Exception;

/**
 * @api
 */
final class UnresolvableConstantException extends Exception
{
    /**
     * @psalm-mutation-free
     */
    public function __construct(public int $class_name, public int $const_name)
    {
    }
}

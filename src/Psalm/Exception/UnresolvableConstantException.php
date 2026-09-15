<?php

declare(strict_types=1);

namespace Psalm\Exception;

use Exception;

final class UnresolvableConstantException extends Exception implements Resultable
{
    /**
     * @psalm-mutation-free
     */
    public function __construct(public string $class_name, public string $const_name)
    {
    }
}

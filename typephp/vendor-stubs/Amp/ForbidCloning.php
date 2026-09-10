<?php

declare(strict_types=1);

namespace Amp;

trait ForbidCloning
{
    protected function __clone()
    {
        throw new \RuntimeException('vendor stub');
    }
}

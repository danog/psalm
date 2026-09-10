<?php

declare(strict_types=1);

namespace Amp;

trait ForbidSerialization
{
    public function __serialize(): array
    {
        return [];
    }
    public function __unserialize(array $data): never
    {
        throw new \RuntimeException('vendor stub');
    }
}

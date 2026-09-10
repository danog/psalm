<?php

declare(strict_types=1);

namespace Amp\Serialization;

interface Serializer
{
    public function serialize($data): string;
    public function unserialize(string $data);
}

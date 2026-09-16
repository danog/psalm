<?php

declare(strict_types=1);

namespace Amp\Serialization;

/**
 * Runtime stubs of amphp/serialization: PHP's native serialize()/unserialize().
 */
interface Serializer
{
    public function serialize(mixed $data): string;

    public function unserialize(string $data): mixed;
}

final class SerializationException extends \Exception
{
}

final class NativeSerializer implements Serializer
{
    public function serialize(mixed $data): string
    {
        return serialize($data);
    }

    public function unserialize(string $data): mixed
    {
        $value = unserialize($data);
        if (!is_array($value) && !is_object($value) && !is_string($value)) {
            throw new SerializationException('Unexpected serialized value');
        }
        return $value;
    }
}

final class IgbinarySerializer implements Serializer
{
    public function serialize(mixed $data): string
    {
        return serialize($data);
    }

    public function unserialize(string $data): mixed
    {
        $value = unserialize($data);
        if (!is_array($value) && !is_object($value) && !is_string($value)) {
            throw new SerializationException('Unexpected serialized value');
        }
        return $value;
    }
}

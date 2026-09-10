<?php

declare(strict_types=1);

namespace Psalm\Internal;

use Amp\Serialization\SerializationException;
use Amp\Serialization\Serializer;
use Override;

use function error_get_last;
use function gzdeflate;
use function gzinflate;

/** @internal */
final class GzipSerializer implements Serializer
{
    /**
     * @psalm-mutation-free
     */
    public function __construct(private readonly Serializer $serializer)
    {
    }

    #[Override]
    public function serialize(array|object|string $data): string
    {
        $data = $this->serializer->serialize($data);
        $data = gzdeflate($data);
        if ($data === false) {
            $error = error_get_last();
            throw new SerializationException('Could not compress data: ' . ($error['message'] ?? 'unknown error'));
        }

        return $data;
    }

    #[Override]
    public function unserialize(string $data): array|object|string
    {
        $data = gzinflate($data);
        if ($data === false) {
            $error = error_get_last();
            throw new SerializationException('Could not decompress data: ' . ($error['message'] ?? 'unknown error'));
        }

        return $this->serializer->unserialize($data);
    }
}

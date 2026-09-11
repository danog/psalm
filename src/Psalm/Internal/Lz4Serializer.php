<?php

declare(strict_types=1);

namespace Psalm\Internal;

use Amp\Serialization\SerializationException;
use Amp\Serialization\Serializer;
use Override;

use function error_get_last;
use function lz4_compress;
use function lz4_uncompress;

/** @internal */
final class Lz4Serializer implements Serializer
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
        /** @var string|false $data */
        $data = lz4_compress($data, 4);
        if ($data === false) {
            $error = error_get_last();
            throw new SerializationException('Could not compress data: ' . ($error['message'] ?? 'unknown error'));
        }

        return $data;
    }

    #[Override]
    /** @return array<string, mixed> */
    public function unserialize(string $data): array|object|string
    {
        /** @var string|false $data */
        $data = lz4_uncompress($data);
        if ($data === false) {
            $error = error_get_last();
            throw new SerializationException('Could not decompress data: ' . ($error['message'] ?? 'unknown error'));
        }

        return $this->serializer->unserialize($data);
    }
}

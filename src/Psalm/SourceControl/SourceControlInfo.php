<?php

declare(strict_types=1);

namespace Psalm\SourceControl;

/**
 * @psalm-immutable
 */
abstract class SourceControlInfo
{
    /** @psalm-mutation-free */
    /** @return array<string, mixed> */
    abstract public function toArray(): array;
}

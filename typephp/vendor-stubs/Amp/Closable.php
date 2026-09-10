<?php

declare(strict_types=1);

namespace Amp;

interface Closable
{
    public function close(): void;
    public function isClosed(): bool;
    public function onClose(\Closure $onClose): void;
}

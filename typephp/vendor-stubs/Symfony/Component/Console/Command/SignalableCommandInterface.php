<?php

declare(strict_types=1);

namespace Symfony\Component\Console\Command;

interface SignalableCommandInterface
{
    public function getSubscribedSignals(): array;
    public function handleSignal(int $signal, int|false $previousExitCode = 0): int|false;
}

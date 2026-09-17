<?php

/** Typed stand-ins for CLI-only dependencies: the port runs single-threaded. */

namespace Fidry\CpuCoreCounter;

final class CpuCoreCounter
{
    public function getCount(): int
    {
        return 1;
    }

    public function getAvailableForParallelisation(): int
    {
        return 1;
    }
}

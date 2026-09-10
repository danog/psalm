<?php

declare(strict_types=1);

namespace Amp\Parallel\Worker;

interface Task
{
    public function run(\Amp\Sync\Channel $channel, \Amp\Cancellation $cancellation): mixed;
}

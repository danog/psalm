<?php

declare(strict_types=1);

namespace Amp\Parallel\Context;

interface Context extends \Amp\Sync\Channel
{
    public function join(?\Amp\Cancellation $cancellation = NULL): mixed;
}

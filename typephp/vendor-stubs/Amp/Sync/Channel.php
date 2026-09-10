<?php

declare(strict_types=1);

namespace Amp\Sync;

interface Channel extends \Amp\Closable
{
    public function receive(?\Amp\Cancellation $cancellation = NULL): mixed;
    public function send(mixed $data): void;
}

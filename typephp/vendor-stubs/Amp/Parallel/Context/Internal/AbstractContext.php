<?php

declare(strict_types=1);

namespace Amp\Parallel\Context\Internal;

abstract class AbstractContext implements \Amp\Parallel\Context\Context
{
    use \Amp\ForbidCloning;
    use \Amp\ForbidSerialization;
    private ?\Amp\Future $result = NULL;
    private readonly \Amp\Sync\Channel $ipcChannel;
    private readonly \Amp\Sync\Channel $resultChannel;
    protected function __construct(\Amp\Sync\Channel $ipcChannel, \Amp\Sync\Channel $resultChannel)
    {
        throw new \RuntimeException('vendor stub');
    }
    public function receive(?\Amp\Cancellation $cancellation = NULL): mixed
    {
        return null;
    }
    public function send(mixed $data): void
    {
        throw new \RuntimeException('vendor stub');
    }
    public function close(): void
    {
        throw new \RuntimeException('vendor stub');
    }
    public function isClosed(): bool
    {
        return false;
    }
    public function onClose(\Closure $onClose): void
    {
        throw new \RuntimeException('vendor stub');
    }
    protected function receiveExitResult(?\Amp\Cancellation $cancellation = NULL): \Amp\Parallel\Context\Internal\ExitResult
    {
        throw new \RuntimeException('vendor stub');
    }
    protected function __clone()
    {
        throw new \RuntimeException('vendor stub');
    }
    public function __serialize(): array
    {
        return [];
    }
    public function __unserialize(array $data): never
    {
        throw new \RuntimeException('vendor stub');
    }
}

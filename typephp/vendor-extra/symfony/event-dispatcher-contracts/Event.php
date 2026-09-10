<?php

namespace Symfony\Contracts\EventDispatcher;

/**
 * Minimal stand-in for symfony/event-dispatcher-contracts, which is not
 * installed: symfony/console's event classes extend it.
 */
class Event
{
    private bool $propagationStopped = false;

    public function isPropagationStopped(): bool
    {
        return $this->propagationStopped;
    }

    public function stopPropagation(): void
    {
        $this->propagationStopped = true;
    }
}

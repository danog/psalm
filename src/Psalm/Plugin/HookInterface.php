<?php

declare(strict_types=1);

namespace Psalm\Plugin;

/**
 * Implemented by every plugin hook interface: a handler object registered with the plugin registration
 * socket is an instance of one or more hook interfaces (the compiled program never looks classes up by name).
 */
interface HookInterface
{
}

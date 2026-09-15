<?php

declare(strict_types=1);

namespace Psalm\Exception;

use Throwable;

/**
 * Marker interface for exceptions that are part of intentional, recoverable error-handling flow (they are, or
 * are meant to be, caught by a `try`/`catch`). The Rust transpiler emits a `throw` of a Resultable exception as
 * a `Result::Err` that propagates to its catch; a `throw` of any NON-Resultable exception (an invariant
 * violation that is never meant to be recovered from) is emitted as a Rust panic (`php_rt::uncaught`), so the
 * enclosing method need not thread `Result` on its account.
 *
 * The transpiler also treats any exception type that actually appears in a `catch` clause as Resultable (see
 * Program::computeResultable), so PHP built-in exceptions (InvalidArgumentException, …) that cannot implement
 * this interface are still handled correctly. This interface is the explicit, forward-looking marker.
 */
interface Resultable extends Throwable
{
}

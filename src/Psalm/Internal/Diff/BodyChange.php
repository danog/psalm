<?php

declare(strict_types=1);

namespace Psalm\Internal\Diff;

/**
 * Carries back whether two matching statements differ in their bodies.
 *
 * A closure type cannot say that one of its parameters is by reference, so the flag travels as an
 * object: the differ hands one to the comparison callback and reads what it set.
 *
 * @internal
 */
final class BodyChange
{
    public bool $changed = false;
}

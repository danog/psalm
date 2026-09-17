<?php

declare(strict_types=1);

namespace Psalm\Storage;

/**
 * @api
 */
final class FunctionStorage extends FunctionLikeStorage
{
    /** @var array<string, bool> */
    public array $byref_uses = [];
}

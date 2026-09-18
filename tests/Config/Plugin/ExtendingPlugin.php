<?php

declare(strict_types=1);

namespace Psalm\Tests\Config\Plugin;

/** Inherits its hook handler from BasePlugin rather than declaring one. */
final class ExtendingPlugin extends BasePlugin
{
}

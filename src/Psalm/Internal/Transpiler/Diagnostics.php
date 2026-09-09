<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node;

use function fwrite;

use const STDERR;

/**
 * Collects transpiler warnings (unsupported constructs) with source locations.
 *
 * @internal
 */
final class Diagnostics
{
    /** @var array<string, int> */
    public array $counts = [];

    /** @var list<string> */
    public array $samples = [];

    public function warn(string $kind, ?Node $node, string $file): void
    {
        $this->counts[$kind] = ($this->counts[$kind] ?? 0) + 1;
        if ($this->counts[$kind] <= 3) {
            $this->samples[] = $kind . ' at ' . $file . ':' . ($node ? $node->getStartLine() : 0);
        }
    }

    public function report(): void
    {
        arsort($this->counts);
        foreach ($this->counts as $kind => $count) {
            fwrite(STDERR, "  [transpiler] $kind: $count\n");
        }
        foreach ($this->samples as $s) {
            fwrite(STDERR, "    e.g. $s\n");
        }
    }
}

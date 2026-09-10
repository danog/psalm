<?php

declare(strict_types=1);

namespace Psalm\Internal\PhpVisitor;

use Override;
use PhpParser;

/**
 * Shifts all nodes in a given AST by a set amount
 *
 * @internal
 */
final class OffsetShifterVisitor extends PhpParser\NodeVisitorAbstract
{
    /**
     * @param array<int, int> $extra_offsets
     * @psalm-mutation-free
     */
    public function __construct(
        private readonly int $file_offset,
        private readonly int $line_offset,
        private array $extra_offsets,
    ) {
    }

    #[Override]
    public function enterNode(PhpParser\Node $node): ?int
    {

        if ($cs = $node->getComments()) {
            $new_comments = [];

            foreach ($cs as $c) {
                if ($c instanceof PhpParser\Comment\Doc) {
                    $new_comments[] = new PhpParser\Comment\Doc(
                        $c->getText(),
                        $c->getStartLine() + $this->line_offset,
                        $c->getStartFilePos() + $this->file_offset + ($this->extra_offsets[$c->getStartFilePos()] ?? 0),
                    );
                } else {
                    $new_comments[] = new PhpParser\Comment(
                        $c->getText(),
                        $c->getStartLine() + $this->line_offset,
                        $c->getStartFilePos() + $this->file_offset + ($this->extra_offsets[$c->getStartFilePos()] ?? 0),
                    );
                }
            }

            $node->attrs()->comments = $new_comments;
        }

        $start_file_pos = $node->getStartFilePos();
        $end_file_pos = $node->getEndFilePos();
        $node->attrs()->startFilePos = $start_file_pos + $this->file_offset + ($this->extra_offsets[$start_file_pos] ?? 0);
        $node->attrs()->endFilePos = $end_file_pos + $this->file_offset + ($this->extra_offsets[$end_file_pos] ?? 0);
        $node->attrs()->startLine = $node->getStartLine() + $this->line_offset;

        return null;
    }
}

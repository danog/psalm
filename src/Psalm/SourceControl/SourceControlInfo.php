<?php

declare(strict_types=1);

namespace Psalm\SourceControl;

/**
 * @psalm-immutable
 * @api
 */
abstract class SourceControlInfo
{
    /** @psalm-mutation-free */
    /**
     * @return array{branch: string, head: array{id: ?string, author_name: ?string, author_email: ?string, committer_name: ?string, committer_email: ?string, message: ?string, date: ?int}, remotes: list<array{name: ?string, url: ?string}>}
     */
    abstract public function toArray(): array;
}

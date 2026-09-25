<?php

declare(strict_types=1);

namespace Psalm\Internal\Provider;

use PhpParser;
use Psalm\Config;


use const DIRECTORY_SEPARATOR;

/** @internal */
final class ParserCacheProvider
{
    /**
     * In-memory cache (the port keeps no persistent cache): file path => [contents hash, statements].
     *
     * @var array<string, list{string, list<PhpParser\Node\Stmt>}>
     */
    private array $items = [];

    public function __construct(Config $config, string $composerLock, bool $persistent = true)
    {
    }

    public function consolidate(): void
    {
    }

    /**
     * @return list<PhpParser\Node\Stmt>|null
     */
    public function loadStatementsFromCache(
        string $file_path,
        ?string $file_content_hash,
    ): ?array {
        if (isset($this->items[$file_path]) && ($file_content_hash === null || $this->items[$file_path][0] === $file_content_hash)) {
            return $this->items[$file_path][1];
        }

        return null;
    }

    public function getHash(string $file_path): ?string
    {
        return isset($this->items[$file_path]) ? $this->items[$file_path][0] : null;
    }

    /**
     * @param  list<PhpParser\Node\Stmt>        $stmts
     */
    public function saveStatementsToCache(
        string $file_path,
        string $file_content_hash,
        array $stmts,
    ): void {
        $this->items[$file_path] = [$file_content_hash, $stmts];
    }
}

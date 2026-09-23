<?php

declare(strict_types=1);

namespace Psalm\Internal\Scanner;

use Override;
use PhpParser;
use PhpParser\NodeTraverser;
use Psalm\Aliases;
use Psalm\Codebase;
use Psalm\FileSource;
use Psalm\Internal\PhpVisitor\ReflectorVisitor;
use Psalm\Progress\Progress;
use Psalm\Progress\VoidProgress;
use Psalm\Storage\FileStorage;

/**
 * @internal
 * @psalm-consistent-constructor
 */
class FileScanner implements FileSource
{
    /**
     * @psalm-mutation-free
     */
    public function __construct(public string $file_path, public string $file_name, public bool $will_analyze)
    {
    }

    /**
     * Whether the file does anything besides declaring things: a statement other than a class, function,
     * include, namespace (looked into), use, declare or comment. Only such a file can affect the context of
     * a file that includes it.
     *
     * @param array<PhpParser\Node\Stmt> $stmts
     */
    private static function hasExtraStatements(array $stmts): bool
    {
        foreach ($stmts as $stmt) {
            if ($stmt instanceof PhpParser\Node\Stmt\Namespace_) {
                if (self::hasExtraStatements($stmt->stmts)) {
                    return true;
                }
                continue;
            }
            if (!$stmt instanceof PhpParser\Node\Stmt\ClassLike
                && !$stmt instanceof PhpParser\Node\Stmt\Function_
                && !$stmt instanceof PhpParser\Node\Stmt\Use_
                && !$stmt instanceof PhpParser\Node\Stmt\GroupUse
                && !$stmt instanceof PhpParser\Node\Stmt\Declare_
                && !$stmt instanceof PhpParser\Node\Stmt\Nop
                && !($stmt instanceof PhpParser\Node\Stmt\Expression
                    && $stmt->expr instanceof PhpParser\Node\Expr\Include_)
            ) {
                return true;
            }
        }
        return false;
    }

    public function scan(
        Codebase $codebase,
        FileStorage $file_storage,
        bool $storage_from_cache = false,
        ?Progress $progress = null,
    ): void {
        if ($progress === null) {
            $progress = new VoidProgress();
        }

        // a cached storage needs no traversal (stub files included: the scanner re-registers their
        // functions and constants from the storage)
        if ((!$this->will_analyze || $file_storage->deep_scan)
            && $storage_from_cache
        ) {
            return;
        }

        $stmts = $codebase->getStatementsForFile(
            $file_storage->file_path,
            $progress,
        );

        $file_storage->has_extra_statements = self::hasExtraStatements($stmts);

        if ($this->will_analyze) {
            $progress->debug('Deep scanning ' . $file_storage->file_path . "\n");
        } else {
            $progress->debug('Scanning ' . $file_storage->file_path . "\n");
        }

        $traverser = new NodeTraverser();
        $traverser->addVisitor(
            new ReflectorVisitor($codebase, $this, $file_storage),
        );

        $traverser->traverse($stmts);

        $file_storage->deep_scan = $this->will_analyze;
    }

    /** @psalm-mutation-free */
    #[Override]
    public function getFilePath(): string
    {
        return $this->file_path;
    }

    /** @psalm-mutation-free */
    #[Override]
    public function getFileName(): string
    {
        return $this->file_name;
    }

    /** @psalm-mutation-free */
    #[Override]
    public function getRootFilePath(): string
    {
        return $this->file_path;
    }

    /** @psalm-mutation-free */
    #[Override]
    public function getRootFileName(): string
    {
        return $this->file_name;
    }

    /**
     * @psalm-pure
     */
    #[Override]
    public function getAliases(): Aliases
    {
        return new Aliases();
    }
}

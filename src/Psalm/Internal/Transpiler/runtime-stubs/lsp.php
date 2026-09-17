<?php

/**
 * Typed stand-in for the language server (its implementation is not transpiled): the members other code
 * reaches through a `LanguageServer` instance, failing loudly if ever run.
 */

namespace Psalm\Internal\LanguageServer;

final class LanguageServer
{
    private function __construct()
    {
    }

    /** @param array<string, mixed> $context */
    public function logError(string $message, array $context = []): void
    {
        throw new \RuntimeException('the language server is not available');
    }

    /** @param array<string, mixed> $context */
    public function logWarning(string $message, array $context = []): void
    {
        throw new \RuntimeException('the language server is not available');
    }

    /** @param array<string, mixed> $context */
    public function logInfo(string $message, array $context = []): void
    {
        throw new \RuntimeException('the language server is not available');
    }

    /** @param array<string, mixed> $context */
    public function logDebug(string $message, array $context = []): void
    {
        throw new \RuntimeException('the language server is not available');
    }

    /** @param list<string> $files */
    public function emitVersionedIssues(array $files, ?int $version = null): void
    {
        throw new \RuntimeException('the language server is not available');
    }

    /** @param list<string> $files */
    public function queueFileAnalysisWithOpenedFiles(array $files = []): void
    {
        throw new \RuntimeException('the language server is not available');
    }
}

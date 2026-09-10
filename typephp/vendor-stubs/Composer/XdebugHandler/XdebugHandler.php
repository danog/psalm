<?php

declare(strict_types=1);

namespace Composer\XdebugHandler;

class XdebugHandler
{
    public const SUFFIX_ALLOW = '_ALLOW_XDEBUG';
    public const SUFFIX_INIS = '_ORIGINAL_INIS';
    public const RESTART_ID = 'internal';
    public const RESTART_SETTINGS = 'XDEBUG_HANDLER_SETTINGS';
    public const DEBUG = 'XDEBUG_HANDLER_DEBUG';
    protected $tmpIni = NULL;
    private static $inRestart = NULL;
    private static $name = NULL;
    private static $skipped = NULL;
    private static $xdebugActive = NULL;
    private static $xdebugMode = NULL;
    private static $xdebugVersion = NULL;
    private $cli = NULL;
    private $debug = NULL;
    private $envAllowXdebug = NULL;
    private $envOriginalInis = NULL;
    private $persistent = NULL;
    private $script = NULL;
    private $statusWriter = NULL;
    public function __construct(string $envPrefix)
    {
        throw new \RuntimeException('vendor stub');
    }
    public function setLogger(\Psr\Log\LoggerInterface $logger): \Composer\XdebugHandler\XdebugHandler
    {
        throw new \RuntimeException('vendor stub');
    }
    public function setMainScript(string $script): \Composer\XdebugHandler\XdebugHandler
    {
        throw new \RuntimeException('vendor stub');
    }
    public function setPersistent(): \Composer\XdebugHandler\XdebugHandler
    {
        throw new \RuntimeException('vendor stub');
    }
    public function check(): void
    {
        throw new \RuntimeException('vendor stub');
    }
    public static function getAllIniFiles(): array
    {
        return [];
    }
    public static function getRestartSettings(): ?array
    {
        return null;
    }
    public static function getSkippedVersion(): string
    {
        return '';
    }
    public static function isXdebugActive(): bool
    {
        return false;
    }
    protected function requiresRestart(bool $default): bool
    {
        return false;
    }
    protected function restart(array $command): void
    {
        throw new \RuntimeException('vendor stub');
    }
    private function doRestart(array $command): void
    {
        throw new \RuntimeException('vendor stub');
    }
    private function prepareRestart(): ?array
    {
        return null;
    }
    private function writeTmpIni(string $tmpFile, array $iniFiles, ?string &$error): bool
    {
        return false;
    }
    private function getCommand(array $argv, string $tmpIni, string $mainScript): array
    {
        return [];
    }
    private function setEnvironment(bool $scannedInis, array $iniFiles, string $tmpIni): bool
    {
        return false;
    }
    private function notify(string $op, ?string $data = NULL): void
    {
        throw new \RuntimeException('vendor stub');
    }
    private function mergeLoadedConfig(array $loadedConfig, array $iniConfig): string
    {
        return '';
    }
    private function checkMainScript(string &$mainScript, array $argv): bool
    {
        return false;
    }
    private function setEnvRestartSettings(array $envArgs): void
    {
        throw new \RuntimeException('vendor stub');
    }
    private function syncSettings(array $settings): void
    {
        throw new \RuntimeException('vendor stub');
    }
    private function checkConfiguration(?string &$info): bool
    {
        return false;
    }
    private function tryEnableSignals(): void
    {
        throw new \RuntimeException('vendor stub');
    }
    private function checkServerArgv(): ?array
    {
        return null;
    }
    private static function setXdebugDetails(): void
    {
        throw new \RuntimeException('vendor stub');
    }
}

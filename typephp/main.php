<?php

declare(strict_types=1);

use Psalm\Internal\Cli\Psalm;

/**
 * Binary entry point for the TypePHP build: TypePHP requires a global main().
 *
 * @param list<string> $argv
 */
function main(int $argc, array $argv): void
{
    Psalm::run($argv);
}

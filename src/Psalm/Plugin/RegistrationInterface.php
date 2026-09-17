<?php

declare(strict_types=1);

namespace Psalm\Plugin;

use Psalm\Plugin\HookInterface;

/**
 * @api
 */
interface RegistrationInterface
{
    public function addStubFile(string $file_name): void;

    /**
     * Registers the hooks a handler object implements (an instance of a class implementing hook interfaces;
     * classes are never looked up by name, the program is compiled).
     */
    public function registerHooksFromClass(HookInterface $handler): void;
}

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
     * Registers the hooks a handler implements: an instance of a class implementing hook interfaces,
     * or -- as plugins written against the upstream API pass it -- that class's name, which is built
     * the way plugin classes are (autoloaded by the interpreted analyzer, from the table of compiled-in
     * plugin classes by the compiled one).
     *
     * @param HookInterface|class-string<HookInterface> $handler
     */
    public function registerHooksFromClass(HookInterface|string $handler): void;
}

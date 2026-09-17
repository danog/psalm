<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\AfterCodebasePopulatedEvent;

/**
 * @api
 */
interface AfterCodebasePopulatedInterface extends HookInterface
{
    /**
     * Called after codebase has been populated
     *
     * @return void
     * @phpcsSuppress SlevomatCodingStandard.TypeHints.ReturnTypeHint
     */
    public static function afterCodebasePopulated(AfterCodebasePopulatedEvent $event);
}

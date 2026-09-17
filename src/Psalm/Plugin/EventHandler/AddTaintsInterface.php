<?php

declare(strict_types=1);

namespace Psalm\Plugin\EventHandler;

use Psalm\Plugin\HookInterface;

use Psalm\Plugin\EventHandler\Event\AddRemoveTaintsEvent;

/**
 * @api
 */
interface AddTaintsInterface extends HookInterface
{
    /**
     * Called to see what taints should be added
     */
    public static function addTaints(AddRemoveTaintsEvent $event): int;
}

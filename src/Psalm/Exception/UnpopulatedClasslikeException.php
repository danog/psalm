<?php

declare(strict_types=1);

namespace Psalm\Exception;

use Psalm\Internal\Interner;

use LogicException;

/**
 * @api
 */
final class UnpopulatedClasslikeException extends LogicException
{
    /**
     * @psalm-mutation-free
     */
    public function __construct(int $fq_classlike_name)
    {
        parent::__construct(
            'Cannot check inheritance - \'' . Interner::lookup($fq_classlike_name) . '\' has not been populated yet.'
            . ' You may need to defer this check to a later phase.',
        );
    }
}

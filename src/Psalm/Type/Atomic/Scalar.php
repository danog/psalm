<?php

declare(strict_types=1);

namespace Psalm\Type\Atomic;

use Override;
use Psalm\Type\Atomic;
use Psalm\Storage\UnserializeMemoryUsageSuppressionTrait;

/**
 * @psalm-immutable
 * @api
 */
abstract class Scalar extends Atomic
{
    use UnserializeMemoryUsageSuppressionTrait;
    /**
     * @psalm-pure
     */
    #[Override]
    public function canBeFullyExpressedInPhp(int $analysis_php_version_id): bool
    {
        return true;
    }
}

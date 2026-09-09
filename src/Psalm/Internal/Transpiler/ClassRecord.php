<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Stmt\ClassLike;
use Psalm\Storage\ClassLikeStorage;

/**
 * @internal
 */
final class ClassRecord
{
    public function __construct(
        public ClassLike $node,
        public ClassLikeStorage $storage,
        public string $file_path,
    ) {
    }
}

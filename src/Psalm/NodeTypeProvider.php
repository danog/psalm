<?php

declare(strict_types=1);

namespace Psalm;

use PhpParser;
use Psalm\Type\Union;

/**
 * @api
 */
interface NodeTypeProvider
{
    public function setType(PhpParser\NodeAbstract $node, Union $type): void;

    public function getType(PhpParser\NodeAbstract $node): ?Union;
}

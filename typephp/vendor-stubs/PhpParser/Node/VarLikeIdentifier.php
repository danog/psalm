<?php

declare(strict_types=1);

namespace PhpParser\Node;

class VarLikeIdentifier extends \PhpParser\Node\Identifier
{
    public function getType(): string
    {
        return '';
    }
}

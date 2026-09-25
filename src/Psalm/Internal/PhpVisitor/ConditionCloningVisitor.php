<?php

declare(strict_types=1);

namespace Psalm\Internal\PhpVisitor;

use Override;
use PhpParser\Node;
use PhpParser\NodeVisitorAbstract;
use Psalm\Internal\Provider\NodeDataProvider;

/**
 * @internal
 */
final class ConditionCloningVisitor extends NodeVisitorAbstract
{
    /**
     * @psalm-mutation-free
     */
    public function __construct(
        private readonly NodeDataProvider $type_provider,
    ) {
    }

    /**
     * A visitor sees every node of the subtree, not just the expression at its root, so this
     * returns the same kind of node it was given.
     */
    #[Override]
    public function enterNode(Node $node): Node
    {
        $origNode = $node;

        $node = clone $node;

        $node_type = $this->type_provider->getType($origNode);

        if ($node_type) {
            $this->type_provider->setType($node, $node_type);
        }

        return $node;
    }
}

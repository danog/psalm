<?php

declare(strict_types=1);

namespace Psalm\Internal\Provider;

use PhpParser\Node;
use Psalm\Storage\Assertion;
use Psalm\Storage\Possibilities;
use Psalm\Type\Union;

/**
 * The node data of a NodeDataProvider, keyed by node object id, shared by the provider's clones. The nodes
 * with data are kept in $nodes so an id is never freed and reused by another node while its data is here.
 *
 * @internal
 */
final class NodeDataStore
{
    /** @var array<int, Union> */
    public array $node_types = [];

    /** @var array<int, list<non-empty-array<string, non-empty-list<non-empty-list<Assertion>>>>|null> */
    public array $node_assertions = [];

    /** @var array<int, array<int, Possibilities>> */
    public array $node_if_true_assertions = [];

    /** @var array<int, array<int, Possibilities>> */
    public array $node_if_false_assertions = [];

    /**
     * The nodes whose data is stored, held so that their object ids (the keys above) cannot be reused by
     * other nodes while the data lives.
     *
     * @var array<int, Node>
     * @psalm-suppress PossiblyUnusedProperty
     */
    public array $nodes = [];
}

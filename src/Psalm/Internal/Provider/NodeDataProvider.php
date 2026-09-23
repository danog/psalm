<?php

declare(strict_types=1);

namespace Psalm\Internal\Provider;

use Override;
use PhpParser\Node;
use PhpParser\Node\Expr;
use PhpParser\Node\Expr\FuncCall;
use PhpParser\Node\Expr\MethodCall;
use PhpParser\Node\Expr\New_;
use PhpParser\Node\Expr\StaticCall;
use PhpParser\NodeAbstract;
use Psalm\NodeTypeProvider;
use Psalm\Storage\Assertion;
use Psalm\Storage\Possibilities;
use Psalm\Type\Union;

use function spl_object_id;

/**
 * @internal
 */
final class NodeDataProvider implements NodeTypeProvider
{
    /**
     * The types and assertions by node, keyed by the node's object id (pzoom keeps a plain map keyed by
     * position; four SplObjectStorage objects cost a method call per read and write). One store object is
     * shared by every clone of the provider, as the SplObjectStorage objects were (a clone is shallow).
     */
    private NodeDataStore $store;

    public bool $cache_assertions = true;

    public function __construct()
    {
        $this->store = new NodeDataStore();
    }

    #[Override]
    public function setType(NodeAbstract $node, Union $type): void
    {
        $id = spl_object_id($node);
        $this->store->node_types[$id] = $type;
        $this->store->nodes[$id] = $node;
    }

    #[Override]
    public function getType(NodeAbstract $node): ?Union
    {
        return $this->store->node_types[spl_object_id($node)] ?? null;
    }

    /**
     * @param list<non-empty-array<string, non-empty-list<non-empty-list<Assertion>>>>|null $assertions
     */
    public function setAssertions(Expr $node, ?array $assertions): void
    {
        if (!$this->cache_assertions) {
            return;
        }

        $id = spl_object_id($node);
        $this->store->node_assertions[$id] = $assertions;
        $this->store->nodes[$id] = $node;
    }

    /**
     * @return list<non-empty-array<string, non-empty-list<non-empty-list<Assertion>>>>|null
     */
    public function getAssertions(Expr $node): ?array
    {
        if (!$this->cache_assertions) {
            return null;
        }

        return $this->store->node_assertions[spl_object_id($node)] ?? null;
    }

    /**
     * @param FuncCall|MethodCall|StaticCall|New_ $node
     * @param array<int, Possibilities>  $assertions
     */
    public function setIfTrueAssertions(Expr $node, array $assertions): void
    {
        $id = spl_object_id($node);
        $this->store->node_if_true_assertions[$id] = $assertions;
        $this->store->nodes[$id] = $node;
    }

    /**
     * @param Expr\FuncCall|MethodCall|StaticCall|New_ $node
     * @return array<int, Possibilities>|null
     */
    public function getIfTrueAssertions(Expr $node): ?array
    {
        return $this->store->node_if_true_assertions[spl_object_id($node)] ?? null;
    }

    /**
     * @param FuncCall|MethodCall|StaticCall|New_ $node
     * @param array<int, Possibilities>  $assertions
     */
    public function setIfFalseAssertions(Expr $node, array $assertions): void
    {
        $id = spl_object_id($node);
        $this->store->node_if_false_assertions[$id] = $assertions;
        $this->store->nodes[$id] = $node;
    }

    /**
     * @param FuncCall|MethodCall|StaticCall|New_ $node
     * @return array<int, Possibilities>|null
     */
    public function getIfFalseAssertions(Expr $node): ?array
    {
        return $this->store->node_if_false_assertions[spl_object_id($node)] ?? null;
    }

    public function isPureCompatible(Expr $node): bool
    {
        $node_type = $this->getType($node);

        return ($node_type && $node_type->reference_free) || ($node->attrs()->pure ?? false);
    }

    public function clearNodeOfTypeAndAssertions(Expr $node): void
    {
        $id = spl_object_id($node);
        unset($this->store->node_types[$id], $this->store->node_assertions[$id]);
    }
}

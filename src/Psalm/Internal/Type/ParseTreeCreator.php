<?php

declare(strict_types=1);

namespace Psalm\Internal\Type;

use Psalm\Exception\TypeParseTreeException;
use Psalm\Internal\Type\ParseTree\CallableParamTree;
use Psalm\Internal\Type\ParseTree\CallableTree;
use Psalm\Internal\Type\ParseTree\CallableWithReturnTypeTree;
use Psalm\Internal\Type\ParseTree\ConditionalTree;
use Psalm\Internal\Type\ParseTree\EncapsulationTree;
use Psalm\Internal\Type\ParseTree\FieldEllipsis;
use Psalm\Internal\Type\ParseTree\GenericTree;
use Psalm\Internal\Type\ParseTree\IndexedAccessTree;
use Psalm\Internal\Type\ParseTree\IntersectionTree;
use Psalm\Internal\Type\ParseTree\KeyedArrayPropertyTree;
use Psalm\Internal\Type\ParseTree\KeyedArrayTree;
use Psalm\Internal\Type\ParseTree\MethodParamTree;
use Psalm\Internal\Type\ParseTree\MethodTree;
use Psalm\Internal\Type\ParseTree\MethodWithReturnTypeTree;
use Psalm\Internal\Type\ParseTree\NullableTree;
use Psalm\Internal\Type\ParseTree\Root;
use Psalm\Internal\Type\ParseTree\TemplateAsTree;
use Psalm\Internal\Type\ParseTree\TemplateIsTree;
use Psalm\Internal\Type\ParseTree\UnionTree;
use Psalm\Internal\Type\ParseTree\Value;

use function array_pop;
use function count;
use function in_array;
use function preg_match;
use function str_contains;
use function strlen;
use function strtolower;
use function substr;

/**
 * @internal
 */
final class ParseTreeCreator
{
    private ParseTree $parse_tree;

    private ParseTree $current_leaf;

    private readonly int $type_token_count;

    private int $t = 0;

    /**
     * @param list<array{0: string, 1: int, 2?: string}> $type_tokens
     */
    public function __construct(private array $type_tokens)
    {
        $this->type_token_count = count($type_tokens);
        $this->parse_tree = new Root();
        $this->current_leaf = $this->parse_tree;
    }

    public function create(): ParseTree
    {
        while ($this->t < $this->type_token_count) {
            $type_token = $this->type_tokens[$this->t];

            switch ($type_token[0]) {
                case '{':
                case ']':
                    throw new TypeParseTreeException('Unexpected token ' . $type_token[0]);

                case '<':
                    $this->handleLessThan();
                    break;

                case '[':
                    $this->handleOpenSquareBracket();
                    break;

                case '(':
                    $this->handleOpenRoundBracket();
                    break;

                case ')':
                    $this->handleClosedRoundBracket();
                    break;

                case '>':
                    do {
                        if ($this->current_leaf->parent === null) {
                            throw new TypeParseTreeException('Cannot parse generic type');
                        }

                        $this->current_leaf = $this->current_leaf->parent;
                    } while (!$this->current_leaf instanceof GenericTree);

                    $this->current_leaf->terminated = true;

                    break;

                case '}':
                    do {
                        if ($this->current_leaf->parent === null) {
                            throw new TypeParseTreeException('Cannot parse array type');
                        }

                        $this->current_leaf = $this->current_leaf->parent;
                    } while (!$this->current_leaf instanceof KeyedArrayTree);

                    $this->current_leaf->terminated = true;

                    break;

                case ',':
                    $this->handleComma();
                    break;

                case '...':
                case '=':
                    $this->handleEllipsisOrEquals($type_token);
                    break;

                case ':':
                    $this->handleColon();
                    break;

                case ' ':
                    $this->handleSpace();
                    break;

                case '?':
                    $this->handleQuestionMark();
                    break;

                case '|':
                    $this->handleBar();
                    break;

                case '&':
                    $this->handleAmpersand();
                    break;

                case 'is':
                case 'as':
                case 'of':
                    $this->handleIsOrAs($type_token);
                    break;

                default:
                    $this->handleValue($type_token);
                    break;
            }

            $this->t++;
        }

        $this->parse_tree->cleanParents();

        if ($this->current_leaf !== $this->parse_tree
            && ($this->parse_tree instanceof GenericTree
                || $this->parse_tree instanceof CallableTree
                || $this->parse_tree instanceof KeyedArrayTree)
        ) {
            throw new TypeParseTreeException(
                'Unterminated bracket',
            );
        }

        return $this->parse_tree;
    }

    /**
     * @param  array{0: string, 1: int, 2?: string} $current_token
     */
    private function createMethodParam(array $current_token, ParseTree $current_parent): void
    {
        $byref = false;
        $variadic = false;
        $has_default = false;
        $default = '';

        if ($current_token[0] === '&') {
            $byref = true;
            ++$this->t;
            $current_token = $this->t < $this->type_token_count ? $this->type_tokens[$this->t] : null;
        } elseif ($current_token[0] === '...') {
            $variadic = true;

            ++$this->t;
            $current_token = $this->t < $this->type_token_count ? $this->type_tokens[$this->t] : null;
        }

        if (!$current_token || $current_token[0][0] !== '$') {
            throw new TypeParseTreeException('Unexpected token after space');
        }

        $new_parent_leaf = new MethodParamTree(
            $current_token[0],
            $byref,
            $variadic,
            $current_parent,
        );

        for ($j = $this->t + 1; $j < $this->type_token_count; ++$j) {
            $ahead_type_token = $this->type_tokens[$j];

            if ($ahead_type_token[0] === ','
                || ($ahead_type_token[0] === ')' && $this->type_tokens[$j - 1][0] !== '(')
            ) {
                $this->t = $j - 1;
                break;
            }

            if ($has_default) {
                $default .= $ahead_type_token[0];
            }

            if ($ahead_type_token[0] === '=') {
                $has_default = true;
                continue;
            }

            if ($j === $this->type_token_count - 1) {
                throw new TypeParseTreeException('Unterminated method');
            }
        }

        $new_parent_leaf->default = $default;

        if ($this->current_leaf !== $current_parent) {
            $new_parent_leaf->children = [$this->current_leaf];
            array_pop($current_parent->children);
        }

        $current_parent->children[] = $new_parent_leaf;

        $this->current_leaf = $new_parent_leaf;
    }

    /**
     * @param  array{0: string, 1: int, 2?: string} $current_token
     */
    private function parseCallableParam(array $current_token, ParseTree $current_parent): void
    {
        $variadic = false;
        $has_default = false;

        if ($current_token[0] === '&') {
            ++$this->t;
            $current_token = $this->t < $this->type_token_count ? $this->type_tokens[$this->t] : null;
        } elseif ($current_token[0] === '...') {
            $variadic = true;

            ++$this->t;
            $current_token = $this->t < $this->type_token_count ? $this->type_tokens[$this->t] : null;
        } elseif ($current_token[0] === '=') {
            $has_default = true;

            ++$this->t;
            $current_token = $this->t < $this->type_token_count ? $this->type_tokens[$this->t] : null;
        }

        if (!$current_token || $current_token[0][0] !== '$' || strlen($current_token[0]) < 2) {
            throw new TypeParseTreeException('Unexpected token after space');
        }

        $new_leaf = new CallableParamTree($current_parent);
        $new_leaf->has_default = $has_default;
        $new_leaf->variadic = $variadic;
        $potential_name = substr($current_token[0], 1);
        if ($potential_name !== '') {
            $new_leaf->name = $potential_name;
        }

        if ($current_parent !== $this->current_leaf) {
            $new_leaf->children = [$this->current_leaf];
            array_pop($current_parent->children);
        }
        $current_parent->children[] = $new_leaf;

        $this->current_leaf = $new_leaf;
    }

    private function handleLessThan(): void
    {
        if (!$this->current_leaf instanceof FieldEllipsis) {
            throw new TypeParseTreeException('Unexpected token <');
        }

        $current_parent = $this->current_leaf->parent;

        if (!$current_parent instanceof KeyedArrayTree) {
            throw new TypeParseTreeException('Unexpected token <');
        }

        array_pop($current_parent->children);

        $generic_leaf = new GenericTree(
            '',
            $current_parent,
        );
        $current_parent->children []= $generic_leaf;

        $this->current_leaf = $generic_leaf;
    }

    private function handleOpenSquareBracket(): void
    {
        if ($this->current_leaf instanceof Root) {
            throw new TypeParseTreeException('Unexpected token [');
        }

        $indexed_access = false;

        $next_token = $this->t + 1 < $this->type_token_count ? $this->type_tokens[$this->t + 1] : null;

        if (!$next_token || $next_token[0] !== ']') {
            $next_next_token = $this->t + 2 < $this->type_token_count ? $this->type_tokens[$this->t + 2] : null;

            if ($next_next_token !== null && $next_next_token[0] === ']') {
                $indexed_access = true;
                ++$this->t;
            } else {
                throw new TypeParseTreeException('Unexpected token [');
            }
        }

        $current_parent = $this->current_leaf->parent;

        if ($indexed_access) {
            if ($next_token === null) {
                throw new TypeParseTreeException('Unexpected token [');
            }

            $new_parent_leaf = new IndexedAccessTree($next_token[0], $current_parent);
        } else {
            if ($this->current_leaf instanceof KeyedArrayPropertyTree) {
                throw new TypeParseTreeException('Unexpected token [');
            }

            $new_parent_leaf = new GenericTree('array', $current_parent);
        }

        $this->current_leaf->parent = $new_parent_leaf;
        $new_parent_leaf->children = [$this->current_leaf];

        if ($current_parent) {
            array_pop($current_parent->children);
            $current_parent->children[] = $new_parent_leaf;
        } else {
            $this->parse_tree = $new_parent_leaf;
        }

        $this->current_leaf = $new_parent_leaf;
        ++$this->t;
    }

    private function handleOpenRoundBracket(): void
    {
        if ($this->current_leaf instanceof Value) {
            throw new TypeParseTreeException('Unrecognised token (');
        }

        $new_parent = !$this->current_leaf instanceof Root ? $this->current_leaf : null;

        $new_leaf = new EncapsulationTree(
            $new_parent,
        );

        if ($this->current_leaf instanceof Root) {
            $this->current_leaf = $this->parse_tree = $new_leaf;
            return;
        }

        if ($new_leaf->parent) {
            $new_leaf->parent->children[] = $new_leaf;
        }

        $this->current_leaf = $new_leaf;
    }

    private function handleClosedRoundBracket(): void
    {
        $prev_token = $this->t > 0 ? $this->type_tokens[$this->t - 1] : null;

        if ($prev_token !== null
            && $prev_token[0] === '('
            && $this->current_leaf instanceof CallableTree
        ) {
            return;
        }

        do {
            if ($this->current_leaf->parent === null) {
                break;
            }

            $this->current_leaf = $this->current_leaf->parent;
        } while (!$this->current_leaf instanceof EncapsulationTree
            && !$this->current_leaf instanceof CallableTree
            && !$this->current_leaf instanceof MethodTree);

        if ($this->current_leaf instanceof EncapsulationTree
            || $this->current_leaf instanceof CallableTree
        ) {
            $this->current_leaf->terminated = true;
        }
    }

    private function handleComma(): void
    {
        if ($this->current_leaf instanceof Root) {
            throw new TypeParseTreeException('Unexpected token ,');
        }

        if (!$this->current_leaf->parent) {
            throw new TypeParseTreeException('Cannot parse comma without a parent node');
        }

        $context_node = $this->current_leaf;

        if ($context_node instanceof GenericTree
            || $context_node instanceof KeyedArrayTree
            || $context_node instanceof CallableTree
            || $context_node instanceof MethodTree
        ) {
            $context_node = $context_node->parent;
        }

        while ($context_node
            && !$context_node instanceof GenericTree
            && !$context_node instanceof KeyedArrayTree
            && !$context_node instanceof CallableTree
            && !$context_node instanceof MethodTree
        ) {
            $context_node = $context_node->parent;
        }

        if (!$context_node) {
            throw new TypeParseTreeException('Cannot parse comma in non-generic/array type');
        }

        $this->current_leaf = $context_node;
    }

    /** @param array{0: string, 1: int, 2?: string} $type_token */
    private function handleEllipsisOrEquals(array $type_token): void
    {
        $prev_token = $this->t > 0 ? $this->type_tokens[$this->t - 1] : null;

        if ($prev_token && ($prev_token[0] === '...' || $prev_token[0] === '=')) {
            throw new TypeParseTreeException('Cannot have duplicate tokens');
        }

        $current_parent = $this->current_leaf->parent;

        if ($this->current_leaf instanceof MethodTree && $type_token[0] === '...') {
            $this->createMethodParam($type_token, $this->current_leaf);
            return;
        }

        if ($this->current_leaf instanceof KeyedArrayTree && $type_token[0] === '...') {
            $leaf = new FieldEllipsis($this->current_leaf);
            $this->current_leaf->children[] = $leaf;
            $this->current_leaf = $leaf;

            return;
        }

        while ($current_parent
            && !$current_parent instanceof CallableTree
            && !$current_parent instanceof CallableParamTree
        ) {
            $this->current_leaf = $current_parent;
            $current_parent = $current_parent->parent;
        }

        if (!$current_parent) {
            if ($type_token[0] === '...') {
                if ($this->current_leaf instanceof CallableTree) {
                    $current_parent = $this->current_leaf;
                } else {
                    throw new TypeParseTreeException('Unexpected token ' . $type_token[0]);
                }
            } else {
                throw new TypeParseTreeException('Unexpected token ' . $type_token[0]);
            }
        }

        if ($current_parent instanceof CallableParamTree) {
            throw new TypeParseTreeException('Cannot have variadic param with a default');
        }

        $new_leaf = new CallableParamTree($current_parent);
        $new_leaf->has_default = $type_token[0] === '=';
        $new_leaf->variadic = $type_token[0] === '...';

        if ($current_parent !== $this->current_leaf) {
            $new_leaf->children = [$this->current_leaf];
            array_pop($current_parent->children);
        }
        $current_parent->children[] = $new_leaf;

        $this->current_leaf = $new_leaf;
    }

    private function handleColon(): void
    {
        if ($this->current_leaf instanceof Root) {
            throw new TypeParseTreeException('Unexpected token :');
        }

        $current_parent = $this->current_leaf->parent;

        if ($this->current_leaf instanceof CallableTree) {
            $new_parent_leaf = new CallableWithReturnTypeTree($current_parent);
            $this->current_leaf->parent = $new_parent_leaf;
            $new_parent_leaf->children = [$this->current_leaf];

            if ($current_parent) {
                array_pop($current_parent->children);
                $current_parent->children[] = $new_parent_leaf;
            } else {
                $this->parse_tree = $new_parent_leaf;
            }

            $this->current_leaf = $new_parent_leaf;
            return;
        }

        if ($this->current_leaf instanceof MethodTree) {
            $new_parent_leaf = new MethodWithReturnTypeTree($current_parent);
            $this->current_leaf->parent = $new_parent_leaf;
            $new_parent_leaf->children = [$this->current_leaf];

            if ($current_parent) {
                array_pop($current_parent->children);
                $current_parent->children[] = $new_parent_leaf;
            } else {
                $this->parse_tree = $new_parent_leaf;
            }

            $this->current_leaf = $new_parent_leaf;
            return;
        }

        if ($current_parent instanceof KeyedArrayPropertyTree) {
            return;
        }

        while (($current_parent instanceof UnionTree
                || $current_parent instanceof CallableWithReturnTypeTree)
            && $this->current_leaf->parent
        ) {
            $this->current_leaf = $this->current_leaf->parent;
            $current_parent = $this->current_leaf->parent;
        }

        if ($current_parent instanceof ConditionalTree) {
            if (count($current_parent->children) > 1) {
                throw new TypeParseTreeException('Cannot process colon in conditional twice');
            }

            $this->current_leaf = $current_parent;
            return;
        }

        if (!$current_parent) {
            throw new TypeParseTreeException('Cannot process colon without parent');
        }

        if (!$this->current_leaf instanceof Value) {
            throw new TypeParseTreeException('Unexpected LHS of property');
        }

        if (!$current_parent instanceof KeyedArrayTree) {
            throw new TypeParseTreeException('Saw : outside of object-like array');
        }

        $prev_token = $this->t > 0 ? $this->type_tokens[$this->t - 1] : null;

        $new_parent_leaf = new KeyedArrayPropertyTree($this->current_leaf->value, $current_parent);
        $new_parent_leaf->possibly_undefined = $prev_token !== null && $prev_token[0] === '?';
        array_pop($current_parent->children);
        $current_parent->children[] = $new_parent_leaf;

        $this->current_leaf = $new_parent_leaf;
    }

    private function handleSpace(): void
    {
        if ($this->current_leaf instanceof Root) {
            throw new TypeParseTreeException('Unexpected space');
        }

        if ($this->current_leaf instanceof KeyedArrayTree) {
            return;
        }

        $current_parent = $this->current_leaf->parent;

        //while ($current_parent && !$method_or_callable_parent) {
        while ($current_parent && !$current_parent instanceof MethodTree && !$current_parent instanceof CallableTree) {
            $this->current_leaf = $current_parent;
            $current_parent = $current_parent->parent;
        }

        $next_token = $this->t + 1 < $this->type_token_count ? $this->type_tokens[$this->t + 1] : null;

        if (!($current_parent instanceof MethodTree || $current_parent instanceof CallableTree) || !$next_token) {
            throw new TypeParseTreeException('Unexpected space');
        }


        if ($current_parent instanceof MethodTree) {
            ++$this->t;
            $this->createMethodParam($next_token, $current_parent);
        }
        if ($current_parent instanceof CallableTree) {
            ++$this->t;
            $this->parseCallableParam($next_token, $current_parent);
        }
    }

    private function handleQuestionMark(): void
    {
        $next_token = $this->t + 1 < $this->type_token_count ? $this->type_tokens[$this->t + 1] : null;

        if ($next_token === null || $next_token[0] !== ':') {
            while (($this->current_leaf instanceof Value
                    || $this->current_leaf instanceof UnionTree
                    || ($this->current_leaf instanceof KeyedArrayTree
                        && $this->current_leaf->terminated)
                    || ($this->current_leaf instanceof GenericTree
                        && $this->current_leaf->terminated)
                    || ($this->current_leaf instanceof EncapsulationTree
                        && $this->current_leaf->terminated)
                    || ($this->current_leaf instanceof CallableTree
                        && $this->current_leaf->terminated)
                    || $this->current_leaf instanceof IntersectionTree)
                && $this->current_leaf->parent
            ) {
                $this->current_leaf = $this->current_leaf->parent;
            }

            if ($this->current_leaf instanceof TemplateIsTree && $this->current_leaf->parent) {
                $current_parent = $this->current_leaf->parent;

                $new_leaf = new ConditionalTree(
                    $this->current_leaf,
                    $this->current_leaf->parent,
                );

                array_pop($current_parent->children);
                $current_parent->children[] = $new_leaf;
                $this->current_leaf = $new_leaf;
            } else {
                $new_parent = !$this->current_leaf instanceof Root ? $this->current_leaf : null;

                if (!$next_token) {
                    throw new TypeParseTreeException('Unexpected token ?');
                }

                $new_leaf = new NullableTree(
                    $new_parent,
                );

                if ($this->current_leaf instanceof Root) {
                    $this->current_leaf = $this->parse_tree = $new_leaf;
                    return;
                }

                if ($new_leaf->parent) {
                    $new_leaf->parent->children[] = $new_leaf;
                }

                $this->current_leaf = $new_leaf;
            }
        }
    }

    private function handleBar(): void
    {
        if ($this->current_leaf instanceof Root) {
            throw new TypeParseTreeException('Unexpected token |');
        }

        $current_parent = $this->current_leaf->parent;

        if ($current_parent instanceof CallableWithReturnTypeTree) {
            $this->current_leaf = $current_parent;
            $current_parent = $current_parent->parent;
        }

        if ($current_parent instanceof NullableTree) {
            $this->current_leaf = $current_parent;
            $current_parent = $current_parent->parent;
        }

        if ($this->current_leaf instanceof UnionTree) {
            throw new TypeParseTreeException('Unexpected token |');
        }

        if ($current_parent instanceof UnionTree) {
            $this->current_leaf = $current_parent;
            return;
        }

        if ($current_parent instanceof IntersectionTree) {
            $this->current_leaf = $current_parent;
            $current_parent = $this->current_leaf->parent;
        }

        if ($current_parent instanceof TemplateIsTree) {
            $new_parent_leaf = new UnionTree($this->current_leaf);
            $new_parent_leaf->children = [$this->current_leaf];
            $new_parent_leaf->parent = $current_parent;
        } else {
            $new_parent_leaf = new UnionTree($current_parent);
            $new_parent_leaf->children = [$this->current_leaf];
        }

        if ($current_parent) {
            array_pop($current_parent->children);
            $current_parent->children[] = $new_parent_leaf;
        } else {
            $this->parse_tree = $new_parent_leaf;
        }

        $this->current_leaf = $new_parent_leaf;
    }

    private function handleAmpersand(): void
    {
        if ($this->current_leaf instanceof Root) {
            throw new TypeParseTreeException(
                'Unexpected &',
            );
        }

        $current_parent = $this->current_leaf->parent;

        if ($current_parent instanceof MethodTree) {
            $this->createMethodParam($this->type_tokens[$this->t], $current_parent);
            return;
        }

        if ($current_parent instanceof IntersectionTree) {
            $this->current_leaf = $current_parent;
            return;
        }

        $new_parent_leaf = new IntersectionTree($current_parent);
        $new_parent_leaf->children = [$this->current_leaf];

        if ($current_parent) {
            array_pop($current_parent->children);
            $current_parent->children[] = $new_parent_leaf;
        } else {
            $this->parse_tree = $new_parent_leaf;
        }

        $this->current_leaf = $new_parent_leaf;
    }

    /** @param array{0: string, 1: int, 2?: string} $type_token */
    private function handleIsOrAs(array $type_token): void
    {
        if ($this->t === 0) {
            $this->handleValue($type_token);
        } else {
            $current_parent = $this->current_leaf->parent;

            if ($current_parent) {
                array_pop($current_parent->children);
            }

            if ($type_token[0] === 'as' || $type_token[0] == 'of') {
                $next_token = $this->t + 1 < $this->type_token_count ? $this->type_tokens[$this->t + 1] : null;

                if (!$this->current_leaf instanceof Value
                    || !$current_parent instanceof GenericTree
                    || !$next_token
                ) {
                    throw new TypeParseTreeException('Unexpected token ' . $type_token[0]);
                }

                $this->current_leaf = new TemplateAsTree(
                    $this->current_leaf->value,
                    $next_token[0],
                    $current_parent,
                );

                $current_parent->children[] = $this->current_leaf;
                ++$this->t;
            } elseif ($this->current_leaf instanceof Value) {
                $this->current_leaf = new TemplateIsTree(
                    $this->current_leaf->value,
                    $current_parent,
                );

                if ($current_parent) {
                    $current_parent->children[] = $this->current_leaf;
                }
            }
        }
    }

    /** @param array{0: string, 1: int, 2?: string} $type_token */
    private function handleValue(array $type_token): void
    {
        $new_parent = !$this->current_leaf instanceof Root ? $this->current_leaf : null;

        if ($this->current_leaf instanceof MethodTree && $type_token[0][0] === '$') {
            $this->createMethodParam($type_token, $this->current_leaf);
            return;
        }

        $next_token = $this->t + 1 < $this->type_token_count ? $this->type_tokens[$this->t + 1] : null;

        switch ($next_token[0] ?? null) {
            case '<':
                $new_leaf = new GenericTree(
                    $type_token[0],
                    $new_parent,
                );
                ++$this->t;
                break;

            case '{':
                ++$this->t;

                $nexter_token = $this->t + 1 < $this->type_token_count ? $this->type_tokens[$this->t + 1] : null;

                if ($nexter_token
                    && str_contains($nexter_token[0], '@')
                    && $type_token[0] !== 'list'
                    && $type_token[0] !== 'array'
                ) {
                    $this->t = $this->type_token_count;
                    if ($type_token[0] === '$this') {
                        $type_token[0] = 'static';
                    }

                    $new_leaf = new Value(
                        $type_token[0],
                        $type_token[1],
                        $type_token[1] + strlen($type_token[0]),
                        $type_token[2] ?? null,
                        $new_parent,
                    );
                    break;
                }

                $new_leaf = new KeyedArrayTree(
                    $type_token[0],
                    $new_parent,
                );

                if ($nexter_token !== null && $nexter_token[0] === '}') {
                    $new_leaf->terminated = true;
                    ++$this->t;
                } elseif ($nexter_token === null) {
                    throw new TypeParseTreeException('Unclosed bracket in keyed array');
                }

                break;

            case '(':
                if (in_array(
                    $type_token[0],
                    ['callable', 'pure-callable', 'Closure', '\Closure', 'pure-Closure'],
                    true,
                )) {
                    $new_leaf = new CallableTree(
                        $type_token[0],
                        $new_parent,
                    );
                } elseif ($type_token[0][0] !== '\\'
                    && $this->current_leaf instanceof Root
                ) {
                    $new_leaf = new MethodTree(
                        $type_token[0],
                        $new_parent,
                    );
                } else {
                    throw new TypeParseTreeException(
                        'Parenthesis must be preceded by “Closure”, “callable”, "pure-callable" or a valid @method'
                        . ' name',
                    );
                }

                ++$this->t;
                break;

            case '::':
                $nexter_token = $this->t + 2 < $this->type_token_count ? $this->type_tokens[$this->t + 2] : null;

                if (!$nexter_token
                    || (!preg_match('/^([a-zA-Z_][a-zA-Z_0-9]*\*?|\*)$/', $nexter_token[0])
                        && strtolower($nexter_token[0]) !== 'class')
                ) {
                    throw new TypeParseTreeException(
                        'Invalid class constant ' . ($nexter_token[0] ?? '<empty>'),
                    );
                }

                $new_leaf = new Value(
                    $type_token[0] . '::' . $nexter_token[0],
                    $type_token[1],
                    $type_token[1] + 2 + strlen($nexter_token[0]),
                    $type_token[2] ?? null,
                    $new_parent,
                );

                $this->t += 2;

                break;

            default:
                if ($type_token[0] === '$this') {
                    $type_token[0] = 'static';
                }

                $new_leaf = new Value(
                    $type_token[0],
                    $type_token[1],
                    $type_token[1] + strlen($type_token[0]),
                    $type_token[2] ?? null,
                    $new_parent,
                );
                break;
        }

        if ($this->current_leaf instanceof Root) {
            $this->current_leaf = $this->parse_tree = $new_leaf;
            return;
        }

        if ($new_leaf->parent) {
            $new_leaf->parent->children[] = $new_leaf;
        }

        $this->current_leaf = $new_leaf;
    }
}

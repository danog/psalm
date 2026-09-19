<?php

declare(strict_types=1);

namespace Psalm\Internal\Analyzer\Statements\Expression\Call\Method;

use Psalm\Internal\MethodIdentifier;
use Psalm\Type\Union;

/**
 * @internal
 */
final class AtomicMethodCallAnalysisResult
{
    public ?Union $return_type = null;

    /**
     * The return type gathered so far.
     *
     * Read through this rather than the property where it was just reset: assigning `null` narrows
     * the property to null, and the analysis that fills it again does not widen it back.
     *
     * @psalm-mutation-free
     */
    public function getReturnType(): ?Union
    {
        return $this->return_type;
    }

    public bool $returns_by_ref = false;

    public bool $has_mock = false;

    public bool $has_valid_method_call_type = false;

    public bool $has_mixed_method_call = false;

    /**
     * @var list<string>
     */
    public array $invalid_method_call_types = [];

    /**
     * @var array<string, bool>
     */
    public array $existent_method_ids = [];

    /**
     * @var list<string>
     */
    public array $non_existent_class_method_ids = [];

    /**
     * @var list<string>
     */
    public array $non_existent_interface_method_ids = [];

    /**
     * @var list<string>
     */
    public array $non_existent_magic_method_ids = [];

    public bool $check_visibility = true;

    public bool $too_many_arguments = true;

    /**
     * @var list<MethodIdentifier>
     */
    public array $too_many_arguments_method_ids = [];

    public bool $too_few_arguments = false;

    /**
     * @var list<MethodIdentifier>
     */
    public array $too_few_arguments_method_ids = [];

    public bool $can_memoize = false;
}

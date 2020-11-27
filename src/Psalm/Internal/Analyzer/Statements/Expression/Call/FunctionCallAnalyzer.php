<?php
namespace Psalm\Internal\Analyzer\Statements\Expression\Call;

use PhpParser;
use PhpParser\BuilderFactory;
use Psalm\Internal\Analyzer\Statements\ExpressionAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\CallAnalyzer;
use Psalm\Internal\Analyzer\StatementsAnalyzer;
use Psalm\Internal\Type\Comparator\CallableTypeComparator;
use Psalm\Internal\Codebase\InternalCallMapHandler;
use Psalm\CodeLocation;
use Psalm\Context;
use Psalm\Internal\Algebra\FormulaGenerator;
use Psalm\Internal\FileManipulation\FileManipulationBuffer;
use Psalm\Internal\DataFlow\TaintSource;
use Psalm\Internal\DataFlow\DataFlowNode;
use Psalm\Internal\Codebase\TaintFlowGraph;
use Psalm\Internal\Type\TypeExpander;
use Psalm\Issue\DeprecatedFunction;
use Psalm\Issue\MixedFunctionCall;
use Psalm\Issue\InvalidFunctionCall;
use Psalm\Issue\ImpureFunctionCall;
use Psalm\Issue\NullFunctionCall;
use Psalm\Issue\PossiblyInvalidFunctionCall;
use Psalm\Issue\PossiblyNullFunctionCall;
use Psalm\Issue\UnusedFunctionCall;
use Psalm\IssueBuffer;
use Psalm\Storage\Assertion;
use Psalm\Storage\FunctionLikeStorage;
use Psalm\Type;
use Psalm\Type\Atomic\TCallable;
use Psalm\Type\Atomic\TCallableObject;
use Psalm\Type\Atomic\TCallableString;
use Psalm\Type\Atomic\TTemplateParam;
use Psalm\Type\Atomic\TMixed;
use Psalm\Type\Atomic\TNamedObject;
use Psalm\Type\Atomic\TNull;
use Psalm\Type\Atomic\TString;
use Psalm\Internal\Algebra;
use Psalm\Type\Reconciler;
use function count;
use function in_array;
use function reset;
use function implode;
use function strtolower;
use function array_merge;
use function array_map;
use function strpos;
use Psalm\Internal\Type\TemplateBound;
use Psalm\Internal\Type\TemplateResult;
use Psalm\Storage\FunctionLikeParameter;
use function explode;

/**
 * @internal
 */
class FunctionCallAnalyzer extends CallAnalyzer
{
    public static function analyze(
        StatementsAnalyzer $statements_analyzer,
        PhpParser\Node\Expr\FuncCall $stmt,
        Context $context
    ) : bool {
        $function_name = $stmt->name;

        $function_id = null;
        $function_params = null;
        $in_call_map = false;

        $is_stubbed = false;

        $function_storage = null;

        $codebase = $statements_analyzer->getCodebase();

        $code_location = new CodeLocation($statements_analyzer->getSource(), $stmt);
        $codebase_functions = $codebase->functions;
        $config = $codebase->config;
        $defined_constants = [];
        $global_variables = [];

        $function_exists = false;

        $real_stmt = $stmt;

        if ($function_name instanceof PhpParser\Node\Name
            && isset($stmt->args[0])
            && !$stmt->args[0]->unpack
        ) {
            $original_function_id = implode('\\', $function_name->parts);

            if ($original_function_id === 'call_user_func') {
                $other_args = \array_slice($stmt->args, 1);

                $function_name = $stmt->args[0]->value;

                $stmt = new PhpParser\Node\Expr\FuncCall(
                    $function_name,
                    $other_args,
                    $stmt->getAttributes()
                );
            }

            if ($original_function_id === 'call_user_func_array' && isset($stmt->args[1])) {
                $function_name = $stmt->args[0]->value;

                $stmt = new PhpParser\Node\Expr\FuncCall(
                    $function_name,
                    [new PhpParser\Node\Arg($stmt->args[1]->value, false, true)],
                    $stmt->getAttributes()
                );
            }
        }

        $byref_uses = [];

        $allow_named_args = true;

        if ($function_name instanceof PhpParser\Node\Expr) {
            [$expr_function_exists, $expr_function_name, $expr_function_params, $byref_uses]
                = self::getAnalyzeNamedExpression(
                    $statements_analyzer,
                    $codebase,
                    $stmt,
                    $real_stmt,
                    $function_name,
                    $context
                );

            if ($expr_function_exists === false) {
                return true;
            }

            if ($expr_function_exists === true) {
                $function_exists = true;
            }

            if ($expr_function_name) {
                $function_name = $expr_function_name;
            }

            if ($expr_function_params) {
                $function_params = $expr_function_params;
            }
        } else {
            $original_function_id = implode('\\', $function_name->parts);

            if (!$function_name instanceof PhpParser\Node\Name\FullyQualified) {
                $function_id = $codebase_functions->getFullyQualifiedFunctionNameFromString(
                    $original_function_id,
                    $statements_analyzer
                );
            } else {
                $function_id = $original_function_id;
            }

            $namespaced_function_exists = $codebase_functions->functionExists(
                $statements_analyzer,
                strtolower($function_id)
            );

            if (!$namespaced_function_exists
                && !$function_name instanceof PhpParser\Node\Name\FullyQualified
            ) {
                $in_call_map = InternalCallMapHandler::inCallMap($original_function_id);
                $is_stubbed = $codebase_functions->hasStubbedFunction($original_function_id);

                if ($is_stubbed || $in_call_map) {
                    $function_id = $original_function_id;
                }
            } else {
                $in_call_map = InternalCallMapHandler::inCallMap($function_id);
                $is_stubbed = $codebase_functions->hasStubbedFunction($function_id);
            }

            if ($is_stubbed || $in_call_map || $namespaced_function_exists) {
                $function_exists = true;
            }

            if ($function_exists
                && $codebase->store_node_types
                && !$context->collect_initializations
                && !$context->collect_mutations
            ) {
                ArgumentMapPopulator::recordArgumentPositions(
                    $statements_analyzer,
                    $stmt,
                    $codebase,
                    $function_id
                );
            }

            $is_predefined = true;

            $is_maybe_root_function = !$function_name instanceof PhpParser\Node\Name\FullyQualified
                && count($function_name->parts) === 1;

            if (!$in_call_map) {
                $predefined_functions = $config->getPredefinedFunctions();
                $is_predefined = isset($predefined_functions[strtolower($original_function_id)])
                    || isset($predefined_functions[strtolower($function_id)]);

                if ($context->check_functions) {
                    if (self::checkFunctionExists(
                        $statements_analyzer,
                        $function_id,
                        $code_location,
                        $is_maybe_root_function
                    ) === false
                    ) {
                        if (ArgumentsAnalyzer::analyze(
                            $statements_analyzer,
                            $stmt->args,
                            null,
                            null,
                            true,
                            $context
                        ) === false) {
                            // fall through
                        }

                        return true;
                    }
                }
            } else {
                $function_exists = true;
            }

            if ($function_exists) {
                $function_params = null;

                if ($codebase->functions->params_provider->has($function_id)) {
                    $function_params = $codebase->functions->params_provider->getFunctionParams(
                        $statements_analyzer,
                        $function_id,
                        $stmt->args,
                        null,
                        $code_location
                    );
                }

                if ($function_params === null) {
                    if (!$in_call_map || $is_stubbed) {
                        try {
                            $function_storage = $codebase_functions->getStorage(
                                $statements_analyzer,
                                strtolower($function_id)
                            );

                            $function_params = $function_storage->params;

                            if (!$function_storage->allow_named_arg_calls) {
                                $allow_named_args = false;
                            }

                            if (!$is_predefined) {
                                $defined_constants = $function_storage->defined_constants;
                                $global_variables = $function_storage->global_variables;
                            }
                        } catch (\UnexpectedValueException $e) {
                            $function_params = [
                                new FunctionLikeParameter('args', false, null, null, null, false, false, true)
                            ];
                        }
                    } else {
                        $function_callable = InternalCallMapHandler::getCallableFromCallMapById(
                            $codebase,
                            $function_id,
                            $stmt->args,
                            $statements_analyzer->node_data
                        );

                        $function_params = $function_callable->params;
                    }
                }

                if ($codebase->store_node_types
                    && !$context->collect_initializations
                    && !$context->collect_mutations
                ) {
                    $codebase->analyzer->addNodeReference(
                        $statements_analyzer->getFilePath(),
                        $function_name,
                        $function_id . '()'
                    );
                }
            }
        }

        $set_inside_conditional = false;

        if ($function_name instanceof PhpParser\Node\Name
            && $function_name->parts === ['assert']
            && !$context->inside_conditional
        ) {
            $context->inside_conditional = true;
            $set_inside_conditional = true;
        }

        if (ArgumentsAnalyzer::analyze(
            $statements_analyzer,
            $stmt->args,
            $function_params,
            $function_id,
            $allow_named_args,
            $context
        ) === false) {
            // fall through
        }

        if ($set_inside_conditional) {
            $context->inside_conditional = false;
        }

        $template_result = null;
        $function_callable = null;

        if ($function_exists) {
            if ($function_name instanceof PhpParser\Node\Name && $function_id) {
                if (!$is_stubbed && $in_call_map) {
                    $function_callable = \Psalm\Internal\Codebase\InternalCallMapHandler::getCallableFromCallMapById(
                        $codebase,
                        $function_id,
                        $stmt->args,
                        $statements_analyzer->node_data
                    );

                    $function_params = $function_callable->params;
                }
            }

            $template_result = new TemplateResult([], []);

            // do this here to allow closure param checks
            if ($function_params !== null
                && ArgumentsAnalyzer::checkArgumentsMatch(
                    $statements_analyzer,
                    $stmt->args,
                    $function_id,
                    $function_params,
                    $function_storage,
                    null,
                    $template_result,
                    $code_location,
                    $context
                ) === false
            ) {
                // fall through
            }

            CallAnalyzer::checkTemplateResult(
                $statements_analyzer,
                $template_result,
                $code_location,
                $function_id
            );

            if ($function_name instanceof PhpParser\Node\Name && $function_id) {
                $stmt_type = self::getFunctionCallReturnType(
                    $statements_analyzer,
                    $codebase,
                    $stmt,
                    $function_name,
                    $function_id,
                    $in_call_map,
                    $is_stubbed,
                    $function_storage,
                    $function_callable,
                    $template_result,
                    $context
                );

                $statements_analyzer->node_data->setType($real_stmt, $stmt_type);

                if ($config->after_every_function_checks) {
                    foreach ($config->after_every_function_checks as $plugin_fq_class_name) {
                        $plugin_fq_class_name::afterEveryFunctionCallAnalysis(
                            $stmt,
                            $function_id,
                            $context,
                            $statements_analyzer->getSource(),
                            $codebase
                        );
                    }
                }
            }

            foreach ($defined_constants as $const_name => $const_type) {
                $context->constants[$const_name] = clone $const_type;
                $context->vars_in_scope[$const_name] = clone $const_type;
            }

            foreach ($global_variables as $var_id => $_) {
                $context->vars_in_scope[$var_id] = Type::getMixed();
                $context->vars_possibly_in_scope[$var_id] = true;
            }

            if ($function_name instanceof PhpParser\Node\Name
                && $function_name->parts === ['assert']
                && isset($stmt->args[0])
            ) {
                self::processAssertFunctionEffects(
                    $statements_analyzer,
                    $codebase,
                    $stmt,
                    $stmt->args[0],
                    $context
                );
            }
        }

        if ($codebase->store_node_types
            && !$context->collect_initializations
            && !$context->collect_mutations
            && ($stmt_type = $statements_analyzer->node_data->getType($real_stmt))
        ) {
            $codebase->analyzer->addNodeType(
                $statements_analyzer->getFilePath(),
                $stmt,
                $stmt_type->getId()
            );
        }

        self::checkFunctionCallPurity(
            $statements_analyzer,
            $codebase,
            $stmt,
            $function_name,
            $function_id,
            $in_call_map,
            $function_storage,
            $context
        );

        if ($function_storage) {
            $inferred_upper_bouunds = $template_result ? $template_result->upper_bounds : [];

            if ($function_storage->assertions && $function_name instanceof PhpParser\Node\Name) {
                self::applyAssertionsToContext(
                    $function_name,
                    null,
                    $function_storage->assertions,
                    $stmt->args,
                    $inferred_upper_bouunds,
                    $context,
                    $statements_analyzer
                );
            }

            if ($function_storage->if_true_assertions) {
                $statements_analyzer->node_data->setIfTrueAssertions(
                    $stmt,
                    array_map(
                        function (Assertion $assertion) use ($inferred_upper_bouunds) : Assertion {
                            return $assertion->getUntemplatedCopy($inferred_upper_bouunds ?: [], null);
                        },
                        $function_storage->if_true_assertions
                    )
                );
            }

            if ($function_storage->if_false_assertions) {
                $statements_analyzer->node_data->setIfFalseAssertions(
                    $stmt,
                    array_map(
                        function (Assertion $assertion) use ($inferred_upper_bouunds) : Assertion {
                            return $assertion->getUntemplatedCopy($inferred_upper_bouunds ?: [], null);
                        },
                        $function_storage->if_false_assertions
                    )
                );
            }

            if ($function_storage->deprecated && $function_id) {
                if (IssueBuffer::accepts(
                    new DeprecatedFunction(
                        'The function ' . $function_id . ' has been marked as deprecated',
                        $code_location,
                        $function_id
                    ),
                    $statements_analyzer->getSuppressedIssues()
                )) {
                    // continue
                }
            }
        }

        if ($byref_uses) {
            foreach ($byref_uses as $byref_use_var => $_) {
                $context->vars_in_scope['$' . $byref_use_var] = Type::getMixed();
                $context->vars_possibly_in_scope['$' . $byref_use_var] = true;
            }
        }

        if ($function_name instanceof PhpParser\Node\Name && $function_id) {
            NamedFunctionCallHandler::handle(
                $statements_analyzer,
                $codebase,
                $stmt,
                $real_stmt,
                $function_name,
                strtolower($function_id),
                $context
            );
        }

        if (!$statements_analyzer->node_data->getType($real_stmt)) {
            $statements_analyzer->node_data->setType($real_stmt, Type::getMixed());
        }

        return true;
    }

    /**
     * @return  array{
     *     ?bool,
     *     ?PhpParser\Node\Expr|PhpParser\Node\Name,
     *     array<int, FunctionLikeParameter>|null,
     *     ?array<string, bool>
     * }
     */
    private static function getAnalyzeNamedExpression(
        StatementsAnalyzer $statements_analyzer,
        \Psalm\Codebase $codebase,
        PhpParser\Node\Expr\FuncCall $stmt,
        PhpParser\Node\Expr\FuncCall $real_stmt,
        PhpParser\Node\Expr $function_name,
        Context $context
    ): array {
        $function_params = null;

        $explicit_function_name = null;
        $function_exists = null;
        $was_in_call = $context->inside_call;
        $context->inside_call = true;

        if (ExpressionAnalyzer::analyze($statements_analyzer, $function_name, $context) === false) {
            $context->inside_call = $was_in_call;

            return [false, null, null, null];
        }

        $context->inside_call = $was_in_call;

        $byref_uses = [];

        if ($stmt_name_type = $statements_analyzer->node_data->getType($function_name)) {
            if ($stmt_name_type->isNull()) {
                if (IssueBuffer::accepts(
                    new NullFunctionCall(
                        'Cannot call function on null value',
                        new CodeLocation($statements_analyzer->getSource(), $stmt)
                    ),
                    $statements_analyzer->getSuppressedIssues()
                )) {
                    // fall through
                }

                return [false, null, null, null];
            }

            if ($stmt_name_type->isNullable()) {
                if (IssueBuffer::accepts(
                    new PossiblyNullFunctionCall(
                        'Cannot call function on possibly null value',
                        new CodeLocation($statements_analyzer->getSource(), $stmt)
                    ),
                    $statements_analyzer->getSuppressedIssues()
                )) {
                    // fall through
                }
            }

            $invalid_function_call_types = [];
            $has_valid_function_call_type = false;

            foreach ($stmt_name_type->getAtomicTypes() as $var_type_part) {
                if ($var_type_part instanceof Type\Atomic\TClosure || $var_type_part instanceof TCallable) {
                    if (!$var_type_part->is_pure && $context->pure) {
                        if (IssueBuffer::accepts(
                            new ImpureFunctionCall(
                                'Cannot call an impure function from a mutation-free context',
                                new CodeLocation($statements_analyzer->getSource(), $stmt)
                            ),
                            $statements_analyzer->getSuppressedIssues()
                        )) {
                            // fall through
                        }
                    }

                    $function_params = $var_type_part->params;

                    if (($stmt_type = $statements_analyzer->node_data->getType($real_stmt))
                        && $var_type_part->return_type
                    ) {
                        $statements_analyzer->node_data->setType(
                            $real_stmt,
                            Type::combineUnionTypes(
                                $stmt_type,
                                $var_type_part->return_type
                            )
                        );
                    } else {
                        $statements_analyzer->node_data->setType(
                            $real_stmt,
                            $var_type_part->return_type ?: Type::getMixed()
                        );
                    }

                    if ($var_type_part instanceof Type\Atomic\TClosure) {
                        $byref_uses += $var_type_part->byref_uses;
                    }

                    $function_exists = true;
                    $has_valid_function_call_type = true;
                } elseif ($var_type_part instanceof TTemplateParam && $var_type_part->as->hasCallableType()) {
                    $has_valid_function_call_type = true;
                } elseif ($var_type_part instanceof TMixed || $var_type_part instanceof TTemplateParam) {
                    $has_valid_function_call_type = true;

                    if (IssueBuffer::accepts(
                        new MixedFunctionCall(
                            'Cannot call function on ' . $var_type_part->getId(),
                            new CodeLocation($statements_analyzer->getSource(), $stmt)
                        ),
                        $statements_analyzer->getSuppressedIssues()
                    )) {
                        // fall through
                    }
                } elseif ($var_type_part instanceof TCallableObject
                    || $var_type_part instanceof TCallableString
                ) {
                    // this is fine
                    $has_valid_function_call_type = true;
                } elseif (($var_type_part instanceof TNamedObject && $var_type_part->value === 'Closure')) {
                    // this is fine
                    $has_valid_function_call_type = true;
                } elseif ($var_type_part instanceof TString
                    || $var_type_part instanceof Type\Atomic\TArray
                    || $var_type_part instanceof Type\Atomic\TList
                    || ($var_type_part instanceof Type\Atomic\TKeyedArray
                        && count($var_type_part->properties) === 2)
                ) {
                    $potential_method_id = null;

                    if ($var_type_part instanceof Type\Atomic\TKeyedArray) {
                        $potential_method_id = CallableTypeComparator::getCallableMethodIdFromTKeyedArray(
                            $var_type_part,
                            $codebase,
                            $context->calling_method_id,
                            $statements_analyzer->getFilePath()
                        );

                        if ($potential_method_id === 'not-callable') {
                            $potential_method_id = null;
                        }
                    } elseif ($var_type_part instanceof Type\Atomic\TLiteralString) {
                        if (!$var_type_part->value) {
                            $invalid_function_call_types[] = '\'\'';
                            continue;
                        }

                        if (strpos($var_type_part->value, '::')) {
                            $parts = explode('::', strtolower($var_type_part->value));
                            $fq_class_name = $parts[0];
                            $fq_class_name = \preg_replace('/^\\\\/', '', $fq_class_name);
                            $potential_method_id = new \Psalm\Internal\MethodIdentifier($fq_class_name, $parts[1]);
                        } else {
                            $explicit_function_name = new PhpParser\Node\Name\FullyQualified(
                                $var_type_part->value,
                                $function_name->getAttributes()
                            );
                        }
                    }

                    if ($potential_method_id) {
                        $codebase->methods->methodExists(
                            $potential_method_id,
                            $context->calling_method_id,
                            null,
                            $statements_analyzer,
                            $statements_analyzer->getFilePath()
                        );
                    }

                    // this is also kind of fine
                    $has_valid_function_call_type = true;
                } elseif ($var_type_part instanceof TNull) {
                    // handled above
                } elseif (!$var_type_part instanceof TNamedObject
                    || !$codebase->classlikes->classOrInterfaceExists($var_type_part->value)
                    || !$codebase->methods->methodExists(
                        new \Psalm\Internal\MethodIdentifier(
                            $var_type_part->value,
                            '__invoke'
                        )
                    )
                ) {
                    $invalid_function_call_types[] = (string)$var_type_part;
                } else {
                    self::analyzeInvokeCall(
                        $statements_analyzer,
                        $stmt,
                        $real_stmt,
                        $function_name,
                        $context,
                        $var_type_part
                    );
                }
            }

            if ($invalid_function_call_types) {
                $var_type_part = reset($invalid_function_call_types);

                if ($has_valid_function_call_type) {
                    if (IssueBuffer::accepts(
                        new PossiblyInvalidFunctionCall(
                            'Cannot treat type ' . $var_type_part . ' as callable',
                            new CodeLocation($statements_analyzer->getSource(), $stmt)
                        ),
                        $statements_analyzer->getSuppressedIssues()
                    )) {
                        // fall through
                    }
                } else {
                    if (IssueBuffer::accepts(
                        new InvalidFunctionCall(
                            'Cannot treat type ' . $var_type_part . ' as callable',
                            new CodeLocation($statements_analyzer->getSource(), $stmt)
                        ),
                        $statements_analyzer->getSuppressedIssues()
                    )) {
                        // fall through
                    }
                }

                return [false, null, null, null];
            }

            if ($statements_analyzer->data_flow_graph instanceof TaintFlowGraph
                && $stmt_name_type->parent_nodes
                && $stmt_name_type->hasString()
                && !\in_array('TaintedInput', $statements_analyzer->getSuppressedIssues())
            ) {
                $arg_location = new CodeLocation($statements_analyzer->getSource(), $function_name);

                $custom_call_sink = \Psalm\Internal\DataFlow\TaintSink::getForMethodArgument(
                    'variable-call',
                    'variable-call',
                    0,
                    $arg_location,
                    $arg_location
                );

                $custom_call_sink->taints = [\Psalm\Type\TaintKind::INPUT_CALLABLE];

                $statements_analyzer->data_flow_graph->addSink($custom_call_sink);

                foreach ($stmt_name_type->parent_nodes as $parent_node) {
                    $statements_analyzer->data_flow_graph->addPath($parent_node, $custom_call_sink, 'call');
                }
            }
        }

        if (!$statements_analyzer->node_data->getType($real_stmt)) {
            $statements_analyzer->node_data->setType($real_stmt, Type::getMixed());
        }

        return [
            $function_exists,
            $explicit_function_name ?: $function_name,
            $function_params,
            $byref_uses
        ];
    }

    private static function analyzeInvokeCall(
        StatementsAnalyzer $statements_analyzer,
        PhpParser\Node\Expr\FuncCall $stmt,
        PhpParser\Node\Expr\FuncCall $real_stmt,
        PhpParser\Node\Expr $function_name,
        Context $context,
        Type\Atomic $atomic_type
    ) : void {
        $old_data_provider = $statements_analyzer->node_data;

        $statements_analyzer->node_data = clone $statements_analyzer->node_data;

        $fake_method_call = new PhpParser\Node\Expr\MethodCall(
            $function_name,
            new PhpParser\Node\Identifier('__invoke', $function_name->getAttributes()),
            $stmt->args
        );

        $suppressed_issues = $statements_analyzer->getSuppressedIssues();

        if (!in_array('PossiblyNullReference', $suppressed_issues, true)) {
            $statements_analyzer->addSuppressedIssues(['PossiblyNullReference']);
        }

        if (!in_array('InternalMethod', $suppressed_issues, true)) {
            $statements_analyzer->addSuppressedIssues(['InternalMethod']);
        }

        if (!in_array('PossiblyInvalidMethodCall', $suppressed_issues, true)) {
            $statements_analyzer->addSuppressedIssues(['PossiblyInvalidMethodCall']);
        }

        $statements_analyzer->node_data->setType($function_name, new Type\Union([$atomic_type]));

        \Psalm\Internal\Analyzer\Statements\Expression\Call\MethodCallAnalyzer::analyze(
            $statements_analyzer,
            $fake_method_call,
            $context,
            false
        );

        if (!in_array('PossiblyNullReference', $suppressed_issues, true)) {
            $statements_analyzer->removeSuppressedIssues(['PossiblyNullReference']);
        }

        if (!in_array('InternalMethod', $suppressed_issues, true)) {
            $statements_analyzer->removeSuppressedIssues(['InternalMethod']);
        }

        if (!in_array('PossiblyInvalidMethodCall', $suppressed_issues, true)) {
            $statements_analyzer->removeSuppressedIssues(['PossiblyInvalidMethodCall']);
        }

        $fake_method_call_type = $statements_analyzer->node_data->getType($fake_method_call);

        $statements_analyzer->node_data = $old_data_provider;

        if ($stmt_type = $statements_analyzer->node_data->getType($real_stmt)) {
            $statements_analyzer->node_data->setType(
                $real_stmt,
                Type::combineUnionTypes(
                    $fake_method_call_type ?: Type::getMixed(),
                    $stmt_type
                )
            );
        } else {
            $statements_analyzer->node_data->setType(
                $real_stmt,
                $fake_method_call_type ?: Type::getMixed()
            );
        }
    }

    private static function processAssertFunctionEffects(
        StatementsAnalyzer $statements_analyzer,
        \Psalm\Codebase $codebase,
        PhpParser\Node\Expr\FuncCall $stmt,
        PhpParser\Node\Arg $first_arg,
        Context $context
    ) : void {
        $first_arg_value_id = \spl_object_id($first_arg->value);

        $assert_clauses = FormulaGenerator::getFormula(
            $first_arg_value_id,
            $first_arg_value_id,
            $first_arg->value,
            $context->self,
            $statements_analyzer,
            $codebase
        );

        \Psalm\Internal\Analyzer\AlgebraAnalyzer::checkForParadox(
            $context->clauses,
            $assert_clauses,
            $statements_analyzer,
            $stmt,
            []
        );

        $simplified_clauses = Algebra::simplifyCNF(array_merge($context->clauses, $assert_clauses));

        $assert_type_assertions = Algebra::getTruthsFromFormula($simplified_clauses);

        $changed_var_ids = [];

        if ($assert_type_assertions) {
            // while in an and, we allow scope to boil over to support
            // statements of the form if ($x && $x->foo())
            $op_vars_in_scope = Reconciler::reconcileKeyedTypes(
                $assert_type_assertions,
                $assert_type_assertions,
                $context->vars_in_scope,
                $changed_var_ids,
                array_map(
                    function ($_): bool {
                        return true;
                    },
                    $assert_type_assertions
                ),
                $statements_analyzer,
                $statements_analyzer->getTemplateTypeMap() ?: [],
                $context->inside_loop,
                new CodeLocation($statements_analyzer->getSource(), $stmt)
            );

            foreach ($changed_var_ids as $var_id => $_) {
                $first_appearance = $statements_analyzer->getFirstAppearance($var_id);

                if ($first_appearance
                    && isset($context->vars_in_scope[$var_id])
                    && $context->vars_in_scope[$var_id]->hasMixed()
                ) {
                    if (!$context->collect_initializations
                        && !$context->collect_mutations
                        && $statements_analyzer->getFilePath() === $statements_analyzer->getRootFilePath()
                        && (!(($parent_source = $statements_analyzer->getSource())
                                    instanceof \Psalm\Internal\Analyzer\FunctionLikeAnalyzer)
                                || !$parent_source->getSource() instanceof \Psalm\Internal\Analyzer\TraitAnalyzer)
                    ) {
                        $codebase->analyzer->decrementMixedCount($statements_analyzer->getFilePath());
                    }

                    IssueBuffer::remove(
                        $statements_analyzer->getFilePath(),
                        'MixedAssignment',
                        $first_appearance->raw_file_start
                    );
                }

                if (isset($op_vars_in_scope[$var_id])) {
                    $op_vars_in_scope[$var_id]->from_docblock = true;
                }
            }

            $context->vars_in_scope = $op_vars_in_scope;
        }

        if ($changed_var_ids) {
            $simplified_clauses = Context::removeReconciledClauses($simplified_clauses, $changed_var_ids)[0];
        }

        $context->clauses = $simplified_clauses;
    }

    /**
     * @param non-empty-string $function_id
     */
    private static function getFunctionCallReturnType(
        StatementsAnalyzer $statements_analyzer,
        \Psalm\Codebase $codebase,
        PhpParser\Node\Expr\FuncCall $stmt,
        PhpParser\Node\Name $function_name,
        string $function_id,
        bool $in_call_map,
        bool $is_stubbed,
        ?FunctionLikeStorage $function_storage,
        ?TCallable $callmap_callable,
        TemplateResult $template_result,
        Context $context
    ) : Type\Union {
        $stmt_type = null;
        $config = $codebase->config;

        if ($codebase->functions->return_type_provider->has($function_id)) {
            $stmt_type = $codebase->functions->return_type_provider->getReturnType(
                $statements_analyzer,
                $function_id,
                $stmt->args,
                $context,
                new CodeLocation($statements_analyzer->getSource(), $function_name)
            );
        }

        if (!$stmt_type) {
            if (!$in_call_map || $is_stubbed) {
                if ($function_storage && $function_storage->template_types) {
                    foreach ($function_storage->template_types as $template_name => $_) {
                        if (!isset($template_result->upper_bounds[$template_name])) {
                            if ($template_name === 'TFunctionArgCount') {
                                $template_result->upper_bounds[$template_name] = [
                                    'fn-' . $function_id => new TemplateBound(
                                        Type::getInt(false, count($stmt->args))
                                    )
                                ];
                            } elseif ($template_name === 'TPhpMajorVersion') {
                                $template_result->upper_bounds[$template_name] = [
                                    'fn-' . $function_id => new TemplateBound(
                                        Type::getInt(false, $codebase->php_major_version)
                                    )
                                ];
                            } else {
                                $template_result->upper_bounds[$template_name] = [
                                    'fn-' . $function_id => new TemplateBound(
                                        Type::getEmpty()
                                    )
                                ];
                            }
                        }
                    }
                }

                if ($function_storage && !$context->isSuppressingExceptions($statements_analyzer)) {
                    $context->mergeFunctionExceptions(
                        $function_storage,
                        new CodeLocation($statements_analyzer->getSource(), $stmt)
                    );
                }

                try {
                    if ($function_storage && $function_storage->return_type) {
                        $return_type = clone $function_storage->return_type;

                        if ($template_result->upper_bounds && $function_storage->template_types) {
                            $return_type = TypeExpander::expandUnion(
                                $codebase,
                                $return_type,
                                null,
                                null,
                                null
                            );

                            $return_type->replaceTemplateTypesWithArgTypes(
                                $template_result,
                                $codebase
                            );
                        }

                        $return_type = TypeExpander::expandUnion(
                            $codebase,
                            $return_type,
                            null,
                            null,
                            null
                        );

                        $return_type_location = $function_storage->return_type_location;

                        if ($config->after_function_checks) {
                            $file_manipulations = [];

                            foreach ($config->after_function_checks as $plugin_fq_class_name) {
                                $plugin_fq_class_name::afterFunctionCallAnalysis(
                                    $stmt,
                                    $function_id,
                                    $context,
                                    $statements_analyzer->getSource(),
                                    $codebase,
                                    $return_type,
                                    $file_manipulations
                                );
                            }

                            if ($file_manipulations) {
                                FileManipulationBuffer::add(
                                    $statements_analyzer->getFilePath(),
                                    $file_manipulations
                                );
                            }
                        }

                        $stmt_type = $return_type;
                        $return_type->by_ref = $function_storage->returns_by_ref;

                        // only check the type locally if it's defined externally
                        if ($return_type_location &&
                            !$is_stubbed && // makes lookups or array_* functions quicker
                            !$config->isInProjectDirs($return_type_location->file_path)
                        ) {
                            $return_type->check(
                                $statements_analyzer,
                                new CodeLocation($statements_analyzer->getSource(), $stmt),
                                $statements_analyzer->getSuppressedIssues(),
                                $context->phantom_classes,
                                true,
                                false,
                                false,
                                $context->calling_method_id
                            );
                        }
                    }
                } catch (\InvalidArgumentException $e) {
                    // this can happen when the function was defined in the Config startup script
                    $stmt_type = Type::getMixed();
                }
            } else {
                if (!$callmap_callable) {
                    throw new \UnexpectedValueException('We should have a callmap callable here');
                }

                $stmt_type = self::getReturnTypeFromCallMapWithArgs(
                    $statements_analyzer,
                    $function_id,
                    $stmt->args,
                    $callmap_callable,
                    $context
                );
            }
        }

        if (!$stmt_type) {
            $stmt_type = Type::getMixed();
        }

        if (!$statements_analyzer->data_flow_graph instanceof TaintFlowGraph || !$function_storage) {
            return $stmt_type;
        }

        $return_node = self::taintReturnType(
            $statements_analyzer,
            $stmt,
            $function_id,
            $function_storage,
            $stmt_type,
            $template_result
        );

        if ($function_storage->proxy_calls !== null) {
            foreach ($function_storage->proxy_calls as $proxy_call) {
                $fake_call_arguments = [];
                foreach ($proxy_call['params'] as $i) {
                    $fake_call_arguments[] = $stmt->args[$i];
                }

                $fake_call_factory = new BuilderFactory();

                if (strpos($proxy_call['fqn'], '::') !== false) {
                    list($fqcn, $method) = explode('::', $proxy_call['fqn']);
                    $fake_call = $fake_call_factory->staticCall($fqcn, $method, $fake_call_arguments);
                } else {
                    $fake_call = $fake_call_factory->funcCall($proxy_call['fqn'], $fake_call_arguments);
                }

                $old_node_data = $statements_analyzer->node_data;
                $statements_analyzer->node_data = clone $statements_analyzer->node_data;

                ExpressionAnalyzer::analyze($statements_analyzer, $fake_call, $context);

                $statements_analyzer->node_data = $old_node_data;

                if ($return_node && $proxy_call['return']) {
                    $fake_call_type = $statements_analyzer->node_data->getType($fake_call);
                    if (null !== $fake_call_type) {
                        foreach ($fake_call_type->parent_nodes as $fake_call_node) {
                            $statements_analyzer->data_flow_graph->addPath($fake_call_node, $return_node, 'return');
                        }
                    }
                }
            }
        }

        return $stmt_type;
    }

    /**
     * @param  list<PhpParser\Node\Arg>   $call_args
     */
    private static function getReturnTypeFromCallMapWithArgs(
        StatementsAnalyzer $statements_analyzer,
        string $function_id,
        array $call_args,
        TCallable $callmap_callable,
        Context $context
    ): Type\Union {
        $call_map_key = strtolower($function_id);

        $codebase = $statements_analyzer->getCodebase();

        if (!$call_args) {
            switch ($call_map_key) {
                case 'hrtime':
                    return new Type\Union([
                        new Type\Atomic\TKeyedArray([
                            Type::getInt(),
                            Type::getInt()
                        ])
                    ]);

                case 'get_called_class':
                    return new Type\Union([
                        new Type\Atomic\TClassString(
                            $context->self ?: 'object',
                            $context->self ? new Type\Atomic\TNamedObject($context->self, true) : null
                        )
                    ]);

                case 'get_parent_class':
                    if ($context->self && $codebase->classExists($context->self)) {
                        $classlike_storage = $codebase->classlike_storage_provider->get($context->self);

                        if ($classlike_storage->parent_classes) {
                            return new Type\Union([
                                new Type\Atomic\TClassString(
                                    \array_values($classlike_storage->parent_classes)[0]
                                )
                            ]);
                        }
                    }
            }
        } else {
            switch ($call_map_key) {
                case 'count':
                    if (($first_arg_type = $statements_analyzer->node_data->getType($call_args[0]->value))) {
                        $atomic_types = $first_arg_type->getAtomicTypes();

                        if (count($atomic_types) === 1) {
                            if (isset($atomic_types['array'])) {
                                if ($atomic_types['array'] instanceof Type\Atomic\TCallableArray
                                    || $atomic_types['array'] instanceof Type\Atomic\TCallableList
                                    || $atomic_types['array'] instanceof Type\Atomic\TCallableKeyedArray
                                ) {
                                    return Type::getInt(false, 2);
                                }

                                if ($atomic_types['array'] instanceof Type\Atomic\TNonEmptyArray) {
                                    return new Type\Union([
                                        $atomic_types['array']->count !== null
                                            ? new Type\Atomic\TLiteralInt($atomic_types['array']->count)
                                            : new Type\Atomic\TInt
                                    ]);
                                }

                                if ($atomic_types['array'] instanceof Type\Atomic\TNonEmptyList) {
                                    return new Type\Union([
                                        $atomic_types['array']->count !== null
                                            ? new Type\Atomic\TLiteralInt($atomic_types['array']->count)
                                            : new Type\Atomic\TInt
                                    ]);
                                }

                                if ($atomic_types['array'] instanceof Type\Atomic\TKeyedArray
                                    && $atomic_types['array']->sealed
                                ) {
                                    return new Type\Union([
                                        new Type\Atomic\TLiteralInt(count($atomic_types['array']->properties))
                                    ]);
                                }
                            }
                        }
                    }

                    break;

                case 'hrtime':
                    if (($first_arg_type = $statements_analyzer->node_data->getType($call_args[0]->value))) {
                        if ((string) $first_arg_type === 'true') {
                            $int = Type::getInt();
                            $int->from_calculation = true;
                            return $int;
                        }

                        if ((string) $first_arg_type === 'false') {
                            return new Type\Union([
                                new Type\Atomic\TKeyedArray([
                                    Type::getInt(),
                                    Type::getInt()
                                ])
                            ]);
                        }

                        return new Type\Union([
                            new Type\Atomic\TKeyedArray([
                                Type::getInt(),
                                Type::getInt()
                            ]),
                            new Type\Atomic\TInt()
                        ]);
                    }

                    $int = Type::getInt();
                    $int->from_calculation = true;
                    return $int;

                case 'min':
                case 'max':
                    if (isset($call_args[0])) {
                        $first_arg = $call_args[0]->value;

                        if ($first_arg_type = $statements_analyzer->node_data->getType($first_arg)) {
                            if ($first_arg_type->hasArray()) {
                                /** @psalm-suppress PossiblyUndefinedStringArrayOffset */
                                $array_type = $first_arg_type->getAtomicTypes()['array'];
                                if ($array_type instanceof Type\Atomic\TKeyedArray) {
                                    return $array_type->getGenericValueType();
                                }

                                if ($array_type instanceof Type\Atomic\TArray) {
                                    return clone $array_type->type_params[1];
                                }

                                if ($array_type instanceof Type\Atomic\TList) {
                                    return clone $array_type->type_param;
                                }
                            } elseif ($first_arg_type->hasScalarType()
                                && ($second_arg = ($call_args[1]->value ?? null))
                                && ($second_arg_type = $statements_analyzer->node_data->getType($second_arg))
                                && $second_arg_type->hasScalarType()
                            ) {
                                return Type::combineUnionTypes($first_arg_type, $second_arg_type);
                            }
                        }
                    }

                    break;

                case 'get_parent_class':
                    // this is unreliable, as it's hard to know exactly what's wanted - attempted this in
                    // https://github.com/vimeo/psalm/commit/355ed831e1c69c96bbf9bf2654ef64786cbe9fd7
                    // but caused problems where it didn’t know exactly what level of child we
                    // were receiving.
                    //
                    // Really this should only work on instances we've created with new Foo(),
                    // but that requires more work
                    break;

                case 'fgetcsv':
                    $string_type = Type::getString();
                    $string_type->addType(new Type\Atomic\TNull);
                    $string_type->ignore_nullable_issues = true;

                    $call_map_return_type = new Type\Union([
                        new Type\Atomic\TNonEmptyList(
                            $string_type
                        ),
                        new Type\Atomic\TFalse,
                        new Type\Atomic\TNull
                    ]);

                    if ($codebase->config->ignore_internal_nullable_issues) {
                        $call_map_return_type->ignore_nullable_issues = true;
                    }

                    if ($codebase->config->ignore_internal_falsable_issues) {
                        $call_map_return_type->ignore_falsable_issues = true;
                    }

                    return $call_map_return_type;
            }
        }

        $stmt_type = $callmap_callable->return_type
            ? clone $callmap_callable->return_type
            : Type::getMixed();

        switch ($function_id) {
            case 'mb_strpos':
            case 'mb_strrpos':
            case 'mb_stripos':
            case 'mb_strripos':
            case 'strpos':
            case 'strrpos':
            case 'stripos':
            case 'strripos':
            case 'strstr':
            case 'stristr':
            case 'strrchr':
            case 'strpbrk':
            case 'array_search':
                break;

            default:
                if ($stmt_type->isFalsable()
                    && $codebase->config->ignore_internal_falsable_issues
                ) {
                    $stmt_type->ignore_falsable_issues = true;
                }
        }

        switch ($call_map_key) {
            case 'array_replace':
            case 'array_replace_recursive':
                if ($codebase->config->ignore_internal_nullable_issues) {
                    $stmt_type->ignore_nullable_issues = true;
                }
                break;
        }

        return $stmt_type;
    }

    private static function taintReturnType(
        StatementsAnalyzer $statements_analyzer,
        PhpParser\Node\Expr\FuncCall $stmt,
        string $function_id,
        FunctionLikeStorage $function_storage,
        Type\Union $stmt_type,
        TemplateResult $template_result
    ) : ?DataFlowNode {
        if (!$statements_analyzer->data_flow_graph instanceof TaintFlowGraph
            || \in_array('TaintedInput', $statements_analyzer->getSuppressedIssues())
        ) {
            return null;
        }

        $node_location = new CodeLocation($statements_analyzer->getSource(), $stmt);

        $function_call_node = DataFlowNode::getForMethodReturn(
            $function_id,
            $function_id,
            $function_storage->signature_return_type_location ?: $function_storage->location,
            $function_storage->specialize_call ? $node_location : null
        );

        $statements_analyzer->data_flow_graph->addNode($function_call_node);

        $codebase = $statements_analyzer->getCodebase();

        $conditionally_removed_taints = [];

        foreach ($function_storage->conditionally_removed_taints as $conditionally_removed_taint) {
            $conditionally_removed_taint = clone $conditionally_removed_taint;

            $conditionally_removed_taint->replaceTemplateTypesWithArgTypes(
                $template_result,
                $codebase
            );

            $expanded_type = TypeExpander::expandUnion(
                $statements_analyzer->getCodebase(),
                $conditionally_removed_taint,
                null,
                null,
                null,
                true,
                true
            );

            foreach ($expanded_type->getLiteralStrings() as $literal_string) {
                $conditionally_removed_taints[] = $literal_string->value;
            }
        }

        if ($conditionally_removed_taints && $function_storage->location) {
            $assignment_node = DataFlowNode::getForAssignment(
                $function_id . '-escaped',
                $function_storage->signature_return_type_location ?: $function_storage->location,
                $function_call_node->specialization_key
            );

            $statements_analyzer->data_flow_graph->addPath(
                $function_call_node,
                $assignment_node,
                'conditionally-escaped',
                [],
                $conditionally_removed_taints
            );

            $stmt_type->parent_nodes[$assignment_node->id] = $assignment_node;
        } else {
            $stmt_type->parent_nodes[$function_call_node->id] = $function_call_node;
        }

        if ($function_storage->return_source_params) {
            $removed_taints = $function_storage->removed_taints;

            if ($function_id === 'preg_replace' && count($stmt->args) > 2) {
                $first_stmt_type = $statements_analyzer->node_data->getType($stmt->args[0]->value);
                $second_stmt_type = $statements_analyzer->node_data->getType($stmt->args[1]->value);

                if ($first_stmt_type
                    && $second_stmt_type
                    && $first_stmt_type->isSingleStringLiteral()
                    && $second_stmt_type->isSingleStringLiteral()
                ) {
                    $first_arg_value = $first_stmt_type->getSingleStringLiteral()->value;

                    $pattern = \substr($first_arg_value, 1, -1);

                    if ($pattern[0] === '['
                        && $pattern[1] === '^'
                        && \substr($pattern, -1) === ']'
                    ) {
                        $pattern = \substr($pattern, 2, -1);

                        if (self::simpleExclusion($pattern, $first_arg_value[0])) {
                            $removed_taints[] = 'html';
                            $removed_taints[] = 'sql';
                        }
                    }
                }
            }

            foreach ($function_storage->return_source_params as $i => $path_type) {
                if (!isset($stmt->args[$i])) {
                    continue;
                }

                $current_arg_is_variadic = $function_storage->params[$i]->is_variadic;
                $taintableArgIndex = [$i];

                if ($current_arg_is_variadic) {
                    $max_params = count($stmt->args) - 1;
                    for ($arg_index = $i + 1; $arg_index <= $max_params; $arg_index++) {
                        $taintableArgIndex[] = $arg_index;
                    }
                }

                foreach ($taintableArgIndex as $argIndex) {
                    $arg_location = new CodeLocation(
                        $statements_analyzer->getSource(),
                        $stmt->args[$argIndex]->value
                    );

                    $function_param_sink = DataFlowNode::getForMethodArgument(
                        $function_id,
                        $function_id,
                        $argIndex,
                        $arg_location,
                        $function_storage->specialize_call ? $node_location : null
                    );

                    $statements_analyzer->data_flow_graph->addNode($function_param_sink);

                    $statements_analyzer->data_flow_graph->addPath(
                        $function_param_sink,
                        $function_call_node,
                        $path_type,
                        $function_storage->added_taints,
                        $removed_taints
                    );
                }
            }
        }

        if ($function_storage->taint_source_types) {
            $method_node = TaintSource::getForMethodReturn(
                $function_id,
                $function_id,
                $node_location
            );

            $method_node->taints = $function_storage->taint_source_types;

            $statements_analyzer->data_flow_graph->addSource($method_node);
        }

        return $function_call_node;
    }

    /**
     * @psalm-pure
     */
    private static function simpleExclusion(string $pattern, string $escape_char) : bool
    {
        $str_length = \strlen($pattern);

        for ($i = 0; $i < $str_length; $i++) {
            $current = $pattern[$i];
            $next = $pattern[$i + 1] ?? null;

            if ($current === '\\') {
                if ($next == null
                    || $next === 'x'
                    || $next === 'u'
                ) {
                    return false;
                }

                if ($next === '.'
                    || $next === '('
                    || $next === ')'
                    || $next === '['
                    || $next === ']'
                    || $next === 's'
                    || $next === 'w'
                    || $next === $escape_char
                ) {
                    $i++;
                    continue;
                }

                return false;
            }

            if ($next !== '-') {
                if ($current === '_'
                    || $current === '-'
                    || $current === '|'
                    || $current === ':'
                    || $current === '#'
                    || $current === '.'
                    || $current === ' '
                ) {
                    continue;
                }

                return false;
            }

            if ($current === ']') {
                return false;
            }

            if (!isset($pattern[$i + 2])) {
                return false;
            }

            if (($current === 'a' && $pattern[$i + 2] === 'z')
                || ($current === 'a' && $pattern[$i + 2] === 'Z')
                || ($current === 'A' && $pattern[$i + 2] === 'Z')
                || ($current === '0' && $pattern[$i + 2] === '9')
            ) {
                $i += 2;
                continue;
            }

            return false;
        }

        return true;
    }

    private static function checkFunctionCallPurity(
        StatementsAnalyzer $statements_analyzer,
        \Psalm\Codebase $codebase,
        PhpParser\Node\Expr\FuncCall $stmt,
        PhpParser\Node $function_name,
        ?string $function_id,
        bool $in_call_map,
        ?FunctionLikeStorage $function_storage,
        Context $context
    ) : void {
        $config = $codebase->config;

        if (!$context->collect_initializations
            && !$context->collect_mutations
            && ($context->mutation_free
                || $context->external_mutation_free
                || $codebase->find_unused_variables
                || !$config->remember_property_assignments_after_call
                || ($statements_analyzer->getSource() instanceof \Psalm\Internal\Analyzer\FunctionLikeAnalyzer
                    && $statements_analyzer->getSource()->track_mutations))
        ) {
            $must_use = true;

            $callmap_function_pure = $function_id && $in_call_map
                ? $codebase->functions->isCallMapFunctionPure(
                    $codebase,
                    $statements_analyzer->node_data,
                    $function_id,
                    $stmt->args,
                    $must_use
                )
                : null;

            if ((!$in_call_map
                    && $function_storage
                    && !$function_storage->pure)
                || ($callmap_function_pure === false)
            ) {
                if ($context->mutation_free || $context->external_mutation_free) {
                    if (IssueBuffer::accepts(
                        new ImpureFunctionCall(
                            'Cannot call an impure function from a mutation-free context',
                            new CodeLocation($statements_analyzer, $function_name)
                        ),
                        $statements_analyzer->getSuppressedIssues()
                    )) {
                        // fall through
                    }
                } elseif ($statements_analyzer->getSource() instanceof \Psalm\Internal\Analyzer\FunctionLikeAnalyzer
                    && $statements_analyzer->getSource()->track_mutations
                ) {
                    $statements_analyzer->getSource()->inferred_has_mutation = true;
                    $statements_analyzer->getSource()->inferred_impure = true;
                }

                if (!$config->remember_property_assignments_after_call) {
                    $context->removeAllObjectVars();
                }
            } elseif ($function_id
                && (($function_storage
                        && $function_storage->pure
                        && !$function_storage->assertions
                        && $must_use)
                    || ($callmap_function_pure === true && $must_use))
                && $codebase->find_unused_variables
                && !$context->inside_conditional
                && !$context->inside_unset
            ) {
                if (!$context->inside_assignment && !$context->inside_call && !$context->inside_use) {
                    if (IssueBuffer::accepts(
                        new UnusedFunctionCall(
                            'The call to ' . $function_id . ' is not used',
                            new CodeLocation($statements_analyzer, $function_name),
                            $function_id
                        ),
                        $statements_analyzer->getSuppressedIssues()
                    )) {
                        // fall through
                    }
                } else {
                    /** @psalm-suppress UndefinedPropertyAssignment */
                    $stmt->pure = true;
                }
            }
        }
    }
}

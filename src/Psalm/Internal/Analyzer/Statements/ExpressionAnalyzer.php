<?php

declare(strict_types=1);

namespace Psalm\Internal\Analyzer\Statements;

use PhpParser;
use Psalm\CodeLocation;
use Psalm\Config;
use Psalm\Context;
use Psalm\Internal\Analyzer\ClosureAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\ArrayAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\AssertionFinder;
use Psalm\Internal\Analyzer\Statements\Expression\AssignmentAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\BinaryOpAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\BitwiseNotAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\BooleanNotAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\Call\FunctionCallAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\Call\MethodCallAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\Call\NewAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\Call\StaticCallAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\CastAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\ClassConstAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\CloneAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\EmptyAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\EncapsulatedStringAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\EvalAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\ExitAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\Fetch\ArrayFetchAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\Fetch\ConstFetchAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\Fetch\InstancePropertyFetchAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\Fetch\StaticPropertyFetchAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\Fetch\VariableFetchAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\GlobalStateAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\IncDecExpressionAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\IncludeAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\InstanceofAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\IssetAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\MagicConstAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\MatchAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\NullsafeAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\PrintAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\TernaryAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\ThrowAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\UnaryPlusMinusAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\YieldAnalyzer;
use Psalm\Internal\Analyzer\Statements\Expression\YieldFromAnalyzer;
use Psalm\Internal\Analyzer\StatementsAnalyzer;
use Psalm\Internal\FileManipulation\FileManipulationBuffer;
use Psalm\Internal\Type\TemplateResult;
use Psalm\Issue\RiskyTruthyFalsyComparison;
use Psalm\Issue\UnrecognizedExpression;
use Psalm\Issue\UnsupportedReferenceUsage;
use Psalm\IssueBuffer;
use Psalm\Node\Expr\VirtualFuncCall;
use Psalm\Node\Scalar\VirtualInterpolatedString;
use Psalm\Node\VirtualArg;
use Psalm\Node\VirtualName;
use Psalm\Plugin\EventHandler\Event\AfterExpressionAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\BeforeExpressionAnalysisEvent;
use Psalm\Type;
use Psalm\Type\Atomic\TBool;

use function assert;
use function count;
use function in_array;
use function strtolower;

/**
 * @internal
 */
final class ExpressionAnalyzer
{
    /**
     * @param bool $assigned_to_reference This is set to true when the expression being analyzed
     *                                    here is being assigned to another variable by reference.
     */
    public static function analyze(
        StatementsAnalyzer $statements_analyzer,
        PhpParser\Node\Expr $stmt,
        Context $context,
        bool $array_assignment = false,
        ?Context $global_context = null,
        ?PhpParser\Node\Stmt $from_stmt = null,
        ?TemplateResult $template_result = null,
        bool $assigned_to_reference = false,
    ): bool {
        if (self::dispatchBeforeExpressionAnalysis($stmt, $context, $statements_analyzer) === false) {
            return false;
        }

        $codebase = $statements_analyzer->getCodebase();

        if (self::handleExpression(
            $statements_analyzer,
            $stmt,
            $context,
            $array_assignment,
            $global_context,
            $from_stmt,
            $template_result,
            $assigned_to_reference,
        ) === false) {
            return false;
        }

        GlobalStateAnalyzer::propagate($statements_analyzer, $stmt, $context);

        if (!$context->inside_conditional
            && ($stmt instanceof PhpParser\Node\Expr\BinaryOp
                || $stmt instanceof PhpParser\Node\Expr\Instanceof_
                || $stmt instanceof PhpParser\Node\Expr\Assign
                || $stmt instanceof PhpParser\Node\Expr\BooleanNot
                || $stmt instanceof PhpParser\Node\Expr\Empty_
                || $stmt instanceof PhpParser\Node\Expr\Isset_
                || $stmt instanceof PhpParser\Node\Expr\FuncCall)
        ) {
            $assertions = $statements_analyzer->node_data->getAssertions($stmt);

            if ($assertions === null) {
                $negate = $context->inside_negation;

                while ($stmt instanceof PhpParser\Node\Expr\BooleanNot) {
                    $stmt = $stmt->expr;
                    $negate = !$negate;
                }

                AssertionFinder::checkAssertionIssues(
                    $stmt,
                    $context->self,
                    $statements_analyzer,
                    $codebase,
                    $negate,
                );
            }
        }

        if (self::dispatchAfterExpressionAnalysis($stmt, $context, $statements_analyzer) === false) {
            return false;
        }

        return true;
    }

    public static function checkRiskyTruthyFalsyComparison(
        Type\Union $type,
        StatementsAnalyzer $statements_analyzer,
        PhpParser\Node\Expr $stmt,
    ): void {
        if (count($type->getAtomicTypes()) > 1) {
            $has_truthy_or_falsy_exclusive_type = false;
            $both_types = $type->getBuilder();
            foreach ($both_types->getAtomicTypes() as $atomic_type) {
                $key = $atomic_type->getKey();
                if ($atomic_type->isTruthy()
                    || $atomic_type->isFalsy()
                    || $atomic_type instanceof TBool) {
                    $both_types->removeType($key);
                    $has_truthy_or_falsy_exclusive_type = true;
                }
            }

            if (count($both_types->getAtomicTypes()) > 0 && $has_truthy_or_falsy_exclusive_type) {
                $both_types = $both_types->freeze();
                IssueBuffer::maybeAdd(
                    new RiskyTruthyFalsyComparison(
                        'Operand of type ' . $type->getId() . ' contains ' .
                        'type' . (count($both_types->getAtomicTypes()) > 1 ? 's' : '') . ' ' .
                        $both_types->getId() . ', which can be falsy and truthy. ' .
                        'This can cause possibly unexpected behavior. Use strict comparison instead.',
                        new CodeLocation($statements_analyzer, $stmt),
                        $type->getId(),
                    ),
                    $statements_analyzer->getSuppressedIssues(),
                );
            }
        }
    }

    /**
     * @param bool $assigned_to_reference This is set to true when the expression being analyzed
     *                                    here is being assigned to another variable by reference.
     */
    /**
     * The kind of expression a node is, by its class: the instanceof chain handleExpression used to walk for
     * every expression is walked once per concrete class (pzoom dispatches with one match).
     *
     * @var array<class-string, int>
     */
    private static array $expression_kinds = [];

    /** @psalm-pure */
    private static function expressionKind(PhpParser\Node\Expr $stmt): int
    {
        if ($stmt instanceof PhpParser\Node\Expr\Variable) {
            return 0;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Assign) {
            return 1;
        }

        if ($stmt instanceof PhpParser\Node\Expr\AssignOp) {
            return 2;
        }

        if ($stmt instanceof PhpParser\Node\Expr\MethodCall) {
            return 3;
        }

        if ($stmt instanceof PhpParser\Node\Expr\StaticCall) {
            return 4;
        }

        if ($stmt instanceof PhpParser\Node\Expr\ConstFetch) {
            return 5;
        }

        if ($stmt instanceof PhpParser\Node\Scalar\String_) {
            return 6;
        }

        if ($stmt instanceof PhpParser\Node\Scalar\MagicConst) {
            return 7;
        }

        if ($stmt instanceof PhpParser\Node\Scalar\Int_) {
            return 8;
        }

        if ($stmt instanceof PhpParser\Node\Scalar\Float_) {
            return 9;
        }

        if ($stmt instanceof PhpParser\Node\Expr\UnaryMinus || $stmt instanceof PhpParser\Node\Expr\UnaryPlus) {
            return 10;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Isset_) {
            return 11;
        }

        if ($stmt instanceof PhpParser\Node\Expr\ClassConstFetch) {
            return 12;
        }

        if ($stmt instanceof PhpParser\Node\Expr\PropertyFetch) {
            return 13;
        }

        if ($stmt instanceof PhpParser\Node\Expr\StaticPropertyFetch) {
            return 14;
        }

        if ($stmt instanceof PhpParser\Node\Expr\BitwiseNot) {
            return 15;
        }

        if ($stmt instanceof PhpParser\Node\Expr\BinaryOp) {
            return 16;
        }

        if ($stmt instanceof PhpParser\Node\Expr\PostInc
            || $stmt instanceof PhpParser\Node\Expr\PostDec
            || $stmt instanceof PhpParser\Node\Expr\PreInc
            || $stmt instanceof PhpParser\Node\Expr\PreDec
        ) {
            return 17;
        }

        if ($stmt instanceof PhpParser\Node\Expr\New_) {
            return 18;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Array_) {
            return 19;
        }

        if ($stmt instanceof PhpParser\Node\Scalar\InterpolatedString) {
            return 20;
        }

        if ($stmt instanceof PhpParser\Node\Expr\FuncCall) {
            return 21;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Ternary) {
            return 22;
        }

        if ($stmt instanceof PhpParser\Node\Expr\BooleanNot) {
            return 23;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Empty_) {
            return 24;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Closure || $stmt instanceof PhpParser\Node\Expr\ArrowFunction) {
            return 25;
        }

        if ($stmt instanceof PhpParser\Node\Expr\ArrayDimFetch) {
            return 26;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Cast) {
            return 27;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Clone_) {
            return 28;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Instanceof_) {
            return 29;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Exit_) {
            return 30;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Include_) {
            return 31;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Eval_) {
            return 32;
        }

        if ($stmt instanceof PhpParser\Node\Expr\AssignRef) {
            return 33;
        }

        if ($stmt instanceof PhpParser\Node\Expr\ErrorSuppress) {
            return 34;
        }

        if ($stmt instanceof PhpParser\Node\Expr\ShellExec) {
            return 35;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Print_) {
            return 36;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Yield_) {
            return 37;
        }

        if ($stmt instanceof PhpParser\Node\Expr\YieldFrom) {
            return 38;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Match_) {
            return 39;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Throw_) {
            return 40;
        }

        if ($stmt instanceof PhpParser\Node\Expr\NullsafePropertyFetch
            || $stmt instanceof PhpParser\Node\Expr\NullsafeMethodCall
        ) {
            return 41;
        }

        if ($stmt instanceof PhpParser\Node\Expr\Error) {
            return 42;
        }
        return -1;
    }

    private static function handleExpression(
        StatementsAnalyzer $statements_analyzer,
        PhpParser\Node\Expr $stmt,
        Context $context,
        bool $array_assignment,
        ?Context $global_context,
        ?PhpParser\Node\Stmt $from_stmt,
        ?TemplateResult $template_result = null,
        bool $assigned_to_reference = false,
    ): bool {
        $kind = self::$expression_kinds[$stmt::class] ??= self::expressionKind($stmt);

        switch ($kind) {
            case 0: // Variable
                assert($stmt instanceof PhpParser\Node\Expr\Variable);
                return VariableFetchAnalyzer::analyze(
                    $statements_analyzer,
                    $stmt,
                    $context,
                    false,
                    null,
                    $array_assignment,
                    false,
                    $assigned_to_reference,
                );
            case 1: // Assign
                assert($stmt instanceof PhpParser\Node\Expr\Assign);
                return self::analyzeAssignment($statements_analyzer, $stmt, $context, $from_stmt);
            case 2: // AssignOp
                assert($stmt instanceof PhpParser\Node\Expr\AssignOp);
                return AssignmentAnalyzer::analyzeAssignmentOperation($statements_analyzer, $stmt, $context);
            case 3: // MethodCall
                assert($stmt instanceof PhpParser\Node\Expr\MethodCall);
                return MethodCallAnalyzer::analyze($statements_analyzer, $stmt, $context, true, $template_result);
            case 4: // StaticCall
                assert($stmt instanceof PhpParser\Node\Expr\StaticCall);
                return StaticCallAnalyzer::analyze($statements_analyzer, $stmt, $context, $template_result);
            case 5: // ConstFetch
                assert($stmt instanceof PhpParser\Node\Expr\ConstFetch);
                ConstFetchAnalyzer::analyze($statements_analyzer, $stmt, $context);

                return true;
            case 6: // String_
                assert($stmt instanceof PhpParser\Node\Scalar\String_);
                $statements_analyzer->node_data->setType($stmt, Type::getString($stmt->value));

                return true;
            case 7: // MagicConst
                assert($stmt instanceof PhpParser\Node\Scalar\MagicConst);
                MagicConstAnalyzer::analyze($statements_analyzer, $stmt, $context);

                return true;
            case 8: // Int_
                assert($stmt instanceof PhpParser\Node\Scalar\Int_);
                $statements_analyzer->node_data->setType($stmt, Type::getInt(false, $stmt->value));

                return true;
            case 9: // Float_
                assert($stmt instanceof PhpParser\Node\Scalar\Float_);
                $statements_analyzer->node_data->setType($stmt, Type::getFloat($stmt->value));

                return true;
            case 10: // UnaryMinus, UnaryPlus
                assert(
                    $stmt instanceof PhpParser\Node\Expr\UnaryMinus
                    || $stmt instanceof PhpParser\Node\Expr\UnaryPlus,
                );
                return UnaryPlusMinusAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 11: // Isset_
                assert($stmt instanceof PhpParser\Node\Expr\Isset_);
                IssetAnalyzer::analyze($statements_analyzer, $stmt, $context);
                $statements_analyzer->node_data->setType($stmt, Type::getBool());

                return true;
            case 12: // ClassConstFetch
                assert($stmt instanceof PhpParser\Node\Expr\ClassConstFetch);
                return ClassConstAnalyzer::analyzeFetch($statements_analyzer, $stmt, $context);
            case 13: // PropertyFetch
                assert($stmt instanceof PhpParser\Node\Expr\PropertyFetch);
                return InstancePropertyFetchAnalyzer::analyze(
                    $statements_analyzer,
                    $stmt,
                    $context,
                    $array_assignment,
                );
            case 14: // StaticPropertyFetch
                assert($stmt instanceof PhpParser\Node\Expr\StaticPropertyFetch);
                return StaticPropertyFetchAnalyzer::analyze(
                    $statements_analyzer,
                    $stmt,
                    $context,
                );
            case 15: // BitwiseNot
                assert($stmt instanceof PhpParser\Node\Expr\BitwiseNot);
                return BitwiseNotAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 16: // BinaryOp
                assert($stmt instanceof PhpParser\Node\Expr\BinaryOp);
                return BinaryOpAnalyzer::analyze(
                    $statements_analyzer,
                    $stmt,
                    $context,
                    0,
                    $from_stmt !== null,
                );
            case 17: // PostInc, PostDec, PreInc, PreDec
                assert(
                    $stmt instanceof PhpParser\Node\Expr\PostInc
                    || $stmt instanceof PhpParser\Node\Expr\PostDec
                    || $stmt instanceof PhpParser\Node\Expr\PreInc
                    || $stmt instanceof PhpParser\Node\Expr\PreDec,
                );
                return IncDecExpressionAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 18: // New_
                assert($stmt instanceof PhpParser\Node\Expr\New_);
                return NewAnalyzer::analyze($statements_analyzer, $stmt, $context, $template_result);
            case 19: // Array_
                assert($stmt instanceof PhpParser\Node\Expr\Array_);
                return ArrayAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 20: // InterpolatedString
                assert($stmt instanceof PhpParser\Node\Scalar\InterpolatedString);
                return EncapsulatedStringAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 21: // FuncCall
                assert($stmt instanceof PhpParser\Node\Expr\FuncCall);
                return FunctionCallAnalyzer::analyze(
                    $statements_analyzer,
                    $stmt,
                    $context,
                    $template_result,
                );
            case 22: // Ternary
                assert($stmt instanceof PhpParser\Node\Expr\Ternary);
                return TernaryAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 23: // BooleanNot
                assert($stmt instanceof PhpParser\Node\Expr\BooleanNot);
                return BooleanNotAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 24: // Empty_
                assert($stmt instanceof PhpParser\Node\Expr\Empty_);
                EmptyAnalyzer::analyze($statements_analyzer, $stmt, $context);

                return true;
            case 25: // Closure, ArrowFunction
                assert(
                    $stmt instanceof PhpParser\Node\Expr\Closure
                    || $stmt instanceof PhpParser\Node\Expr\ArrowFunction,
                );
                return ClosureAnalyzer::analyzeExpression($statements_analyzer, $stmt, $context);
            case 26: // ArrayDimFetch
                assert($stmt instanceof PhpParser\Node\Expr\ArrayDimFetch);
                return ArrayFetchAnalyzer::analyze(
                    $statements_analyzer,
                    $stmt,
                    $context,
                );
            case 27: // Cast
                assert($stmt instanceof PhpParser\Node\Expr\Cast);
                return CastAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 28: // Clone_
                assert($stmt instanceof PhpParser\Node\Expr\Clone_);
                return CloneAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 29: // Instanceof_
                assert($stmt instanceof PhpParser\Node\Expr\Instanceof_);
                return InstanceofAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 30: // Exit_
                assert($stmt instanceof PhpParser\Node\Expr\Exit_);
                return ExitAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 31: // Include_
                assert($stmt instanceof PhpParser\Node\Expr\Include_);
                return IncludeAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 32: // Eval_
                assert($stmt instanceof PhpParser\Node\Expr\Eval_);
                EvalAnalyzer::analyze($statements_analyzer, $stmt, $context);
                return true;
            case 33: // AssignRef
                assert($stmt instanceof PhpParser\Node\Expr\AssignRef);
                if (!AssignmentAnalyzer::analyzeAssignmentRef($statements_analyzer, $stmt, $context, $from_stmt)) {
                    IssueBuffer::maybeAdd(
                        new UnsupportedReferenceUsage(
                            "This reference cannot be analyzed by Psalm",
                            new CodeLocation($statements_analyzer->getSource(), $stmt),
                        ),
                        $statements_analyzer->getSuppressedIssues(),
                    );

                    // Analyze as if it were a normal assignment and just pretend the reference doesn't exist
                    return self::analyzeAssignment($statements_analyzer, $stmt, $context, $from_stmt);
                }

                return true;
            case 34: // ErrorSuppress
                assert($stmt instanceof PhpParser\Node\Expr\ErrorSuppress);
                $context->error_suppressing = true;
                if (self::analyze($statements_analyzer, $stmt->expr, $context) === false) {
                    return false;
                }
                $context->error_suppressing = false;

                $expr_type = $statements_analyzer->node_data->getType($stmt->expr);

                if ($expr_type) {
                    $statements_analyzer->node_data->setType($stmt, $expr_type);
                }

                return true;
            case 35: // ShellExec
                assert($stmt instanceof PhpParser\Node\Expr\ShellExec);
                $concat = new VirtualInterpolatedString($stmt->parts, $stmt->getAttributes());
                $virtual_call = new VirtualFuncCall(new VirtualName(['shell_exec']), [
                    new VirtualArg($concat),
                ], $stmt->getAttributes());
                return self::handleExpression(
                    $statements_analyzer,
                    $virtual_call,
                    $context,
                    $array_assignment,
                    $global_context,
                    $from_stmt,
                    $template_result,
                    $assigned_to_reference,
                );
            case 36: // Print_
                assert($stmt instanceof PhpParser\Node\Expr\Print_);
                $was_inside_call = $context->inside_call;
                $context->inside_call = true;
                if (PrintAnalyzer::analyze($statements_analyzer, $stmt, $context) === false) {
                    $context->inside_call = $was_inside_call;

                    return false;
                }
                $context->inside_call = $was_inside_call;

                return true;
            case 37: // Yield_
                assert($stmt instanceof PhpParser\Node\Expr\Yield_);
                return YieldAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 38: // YieldFrom
                assert($stmt instanceof PhpParser\Node\Expr\YieldFrom);
                return YieldFromAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 39: // Match_
                assert($stmt instanceof PhpParser\Node\Expr\Match_);
                if ($statements_analyzer->getCodebase()->analysis_php_version_id >= 8_00_00) {
                    return MatchAnalyzer::analyze($statements_analyzer, $stmt, $context);
                }
                break;
            case 40: // Throw_
                assert($stmt instanceof PhpParser\Node\Expr\Throw_);
                return ThrowAnalyzer::analyze($statements_analyzer, $stmt, $context);
            case 41: // NullsafePropertyFetch, NullsafeMethodCall
                assert(
                    $stmt instanceof PhpParser\Node\Expr\NullsafePropertyFetch
                    || $stmt instanceof PhpParser\Node\Expr\NullsafeMethodCall,
                );
                if ($statements_analyzer->getCodebase()->analysis_php_version_id >= 8_00_00) {
                    return NullsafeAnalyzer::analyze($statements_analyzer, $stmt, $context);
                }
                break;
            case 42: // Error
                assert($stmt instanceof PhpParser\Node\Expr\Error);
                // do nothing
                return true;
        }

        $codebase = $statements_analyzer->getCodebase();
        IssueBuffer::maybeAdd(
            new UnrecognizedExpression(
                'Psalm does not understand ' . $stmt::class . ' for PHP ' .
                $codebase->getMajorAnalysisPhpVersion() . '.' . $codebase->getMinorAnalysisPhpVersion(),
                new CodeLocation($statements_analyzer->getSource(), $stmt),
            ),
            $statements_analyzer->getSuppressedIssues(),
        );

        return false;
    }

    /**
     * @psalm-external-mutation-free
     */
    public static function isMock(string $fq_class_name): bool
    {
        return in_array(strtolower($fq_class_name), Config::getInstance()->getMockClasses(), true);
    }

    /**
     * @param PhpParser\Node\Expr\Assign|PhpParser\Node\Expr\AssignRef $stmt
     */
    private static function analyzeAssignment(
        StatementsAnalyzer $statements_analyzer,
        PhpParser\Node\Expr $stmt,
        Context $context,
        ?PhpParser\Node\Stmt $from_stmt,
    ): bool {
        $assignment_type = AssignmentAnalyzer::analyze(
            $statements_analyzer,
            $stmt->var,
            $stmt->expr,
            null,
            $context,
            $stmt->getDocComment() ?? $from_stmt?->getDocComment(),
            [],
            !$from_stmt ? $stmt : null,
        );

        if ($assignment_type === null) {
            return false;
        }

        if (!$from_stmt) {
            $statements_analyzer->node_data->setType($stmt, $assignment_type);
        }

        return true;
    }

    private static function dispatchBeforeExpressionAnalysis(
        PhpParser\Node\Expr $expr,
        Context $context,
        StatementsAnalyzer $statements_analyzer,
    ): ?bool {
        $codebase = $statements_analyzer->getCodebase();

        if ($codebase->config->eventDispatcher->before_expression_checks === []) {
            return null;
        }

        $event = new BeforeExpressionAnalysisEvent(
            $expr,
            $context,
            $statements_analyzer,
            $codebase,
            [],
        );

        if ($codebase->config->eventDispatcher->dispatchBeforeExpressionAnalysis($event) === false) {
            return false;
        }

        $file_manipulations = $event->getFileReplacements();

        if ($file_manipulations !== []) {
            FileManipulationBuffer::add($statements_analyzer->getFilePath(), $file_manipulations);
        }

        return null;
    }

    private static function dispatchAfterExpressionAnalysis(
        PhpParser\Node\Expr $expr,
        Context $context,
        StatementsAnalyzer $statements_analyzer,
    ): ?bool {
        $codebase = $statements_analyzer->getCodebase();

        if ($codebase->config->eventDispatcher->after_expression_checks === []) {
            return null;
        }

        $event = new AfterExpressionAnalysisEvent(
            $expr,
            $context,
            $statements_analyzer,
            $codebase,
            [],
        );

        if ($codebase->config->eventDispatcher->dispatchAfterExpressionAnalysis($event) === false) {
            return false;
        }

        $file_manipulations = $event->getFileReplacements();

        if ($file_manipulations !== []) {
            FileManipulationBuffer::add($statements_analyzer->getFilePath(), $file_manipulations);
        }

        return null;
    }
}

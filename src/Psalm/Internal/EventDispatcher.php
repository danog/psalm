<?php

declare(strict_types=1);

namespace Psalm\Internal;

use Psalm\Plugin\EventHandler\AddTaintsInterface;
use Psalm\Plugin\EventHandler\AfterAnalysisInterface;
use Psalm\Plugin\EventHandler\AfterClassLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\AfterClassLikeExistenceCheckInterface;
use Psalm\Plugin\EventHandler\AfterClassLikeVisitInterface;
use Psalm\Plugin\EventHandler\AfterCodebasePopulatedInterface;
use Psalm\Plugin\EventHandler\AfterEveryFunctionCallAnalysisInterface;
use Psalm\Plugin\EventHandler\AfterExpressionAnalysisInterface;
use Psalm\Plugin\EventHandler\AfterFileAnalysisInterface;
use Psalm\Plugin\EventHandler\AfterFunctionCallAnalysisInterface;
use Psalm\Plugin\EventHandler\AfterFunctionLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\AfterMethodCallAnalysisInterface;
use Psalm\Plugin\EventHandler\AfterStatementAnalysisInterface;
use Psalm\Plugin\EventHandler\BeforeAddIssueInterface;
use Psalm\Plugin\EventHandler\BeforeExpressionAnalysisInterface;
use Psalm\Plugin\EventHandler\BeforeFileAnalysisInterface;
use Psalm\Plugin\EventHandler\BeforeStatementAnalysisInterface;
use Psalm\Plugin\EventHandler\ClassFilePathProviderInterface;
use Psalm\Plugin\EventHandler\Event\AddRemoveTaintsEvent;
use Psalm\Plugin\EventHandler\Event\AfterAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeExistenceCheckEvent;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeVisitEvent;
use Psalm\Plugin\EventHandler\Event\AfterCodebasePopulatedEvent;
use Psalm\Plugin\EventHandler\Event\AfterEveryFunctionCallAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\AfterExpressionAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\AfterFileAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\AfterFunctionCallAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\AfterFunctionLikeAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\AfterMethodCallAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\AfterStatementAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\BeforeAddIssueEvent;
use Psalm\Plugin\EventHandler\Event\BeforeExpressionAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\BeforeFileAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\BeforeStatementAnalysisEvent;
use Psalm\Plugin\EventHandler\Event\StringInterpreterEvent;
use Psalm\Plugin\EventHandler\RemoveTaintsInterface;
use Psalm\Plugin\EventHandler\StringInterpreterInterface;
use Psalm\Type\Atomic\TLiteralString;

use function count;
use function is_bool;

/**
 * @internal
 */
final class EventDispatcher
{
    /**
     * Static methods to be called after method checks have completed
     *
     * @var list<AfterMethodCallAnalysisInterface>
     */
    private array $after_method_checks = [];

    /**
     * Static methods to be called after project function checks have completed
     *
     * Called after function calls to functions defined in the project.
     *
     * Allows influencing the return type and adding of modifications.
     *
     * @var list<AfterFunctionCallAnalysisInterface>
     */
    public array $after_function_checks = [];

    /**
     * Static methods to be called after every function call
     *
     * Called after each function call, including php internal functions.
     *
     * Cannot change the call or influence its return type
     *
     * @var list<AfterEveryFunctionCallAnalysisInterface>
     */
    public array $after_every_function_checks = [];

    /**
     * Static methods to be called before expression checks are completed
     *
     * @var list<BeforeExpressionAnalysisInterface>
     */
    public array $before_expression_checks = [];

    /**
     * Static methods to be called after expression checks have completed
     *
     * @var list<AfterExpressionAnalysisInterface>
     */
    public array $after_expression_checks = [];

    /**
     * Static methods to be called before statement checks are processed
     *
     * @var list<BeforeStatementAnalysisInterface>
     */
    public array $before_statement_checks = [];

    /**
     * Static methods to be called after statement checks have completed
     *
     * @var list<AfterStatementAnalysisInterface>
     */
    public array $after_statement_checks = [];

    /**
     * Static methods to be called after method checks have completed
     *
     * @var list<StringInterpreterInterface>
     */
    public array $string_interpreters = [];

    /**
     * Static methods to be called after classlike exists checks have completed
     *
     * @var list<AfterClassLikeExistenceCheckInterface>
     */
    public array $after_classlike_exists_checks = [];

    /**
     * Static methods to be called after classlike checks have completed
     *
     * @var list<AfterClassLikeAnalysisInterface>
     */
    public array $after_classlike_checks = [];

    /**
     * Static methods to be called after classlikes have been scanned
     *
     * @var list<AfterClassLikeVisitInterface>
     */
    private array $after_visit_classlikes = [];

    /**
     * Static methods to be called after codebase has been populated
     *
     * @var list<AfterCodebasePopulatedInterface>
     */
    public array $after_codebase_populated = [];

    /**
     * @var list<BeforeAddIssueInterface>
     */
    private array $before_add_issue = [];

    /**
     * Static methods to be called after codebase has been populated
     *
     * @var list<AfterAnalysisInterface>
     */
    public array $after_analysis = [];

    /**
     * Static methods to be called after a file has been analyzed
     *
     * @var list<AfterFileAnalysisInterface>
     */
    public array $after_file_checks = [];

    /**
     * Static methods to be called before a file is analyzed
     *
     * @var list<BeforeFileAnalysisInterface>
     */
    public array $before_file_checks = [];

    /**
     * Static methods to be called to determine the file path a of a class
     * not autoloadable using composer, but autoloadable through another autoloader
     * (to avoid actually autoloading and parsing the class).
     *
     * @var list<ClassFilePathProviderInterface>
     */
    public array $file_path_provider_interface = [];

    /**
     * Static methods to be called after functionlike checks have completed
     *
     * @var list<AfterFunctionLikeAnalysisInterface>
     */
    public array $after_functionlike_checks = [];

    /**
     * Static methods to be called to see if taints should be added
     *
     * @var list<AddTaintsInterface>
     */
    public array $add_taints_checks = [];

    /**
     * Static methods to be called to see if taints should be removed
     *
     * @var list<RemoveTaintsInterface>
     */
    public array $remove_taints_checks = [];

    /**
     * Registers the hooks a handler object implements.
     *
     * @psalm-external-mutation-free
     */
    public function registerClass(object $class): void
    {
        if ($class instanceof AfterMethodCallAnalysisInterface) {
            $this->after_method_checks[] = $class;
        }

        if ($class instanceof AfterFunctionCallAnalysisInterface) {
            $this->after_function_checks[] = $class;
        }

        if ($class instanceof AfterEveryFunctionCallAnalysisInterface) {
            $this->after_every_function_checks[] = $class;
        }

        if ($class instanceof BeforeExpressionAnalysisInterface) {
            $this->before_expression_checks[] = $class;
        }

        if ($class instanceof AfterExpressionAnalysisInterface) {
            $this->after_expression_checks[] = $class;
        }

        if ($class instanceof BeforeStatementAnalysisInterface) {
            $this->before_statement_checks[] = $class;
        }

        if ($class instanceof AfterStatementAnalysisInterface) {
            $this->after_statement_checks[] = $class;
        }

        if ($class instanceof StringInterpreterInterface) {
            $this->string_interpreters[] = $class;
        }

        if ($class instanceof AfterClassLikeExistenceCheckInterface) {
            $this->after_classlike_exists_checks[] = $class;
        }

        if ($class instanceof AfterClassLikeAnalysisInterface) {
            $this->after_classlike_checks[] = $class;
        }

        if ($class instanceof AfterClassLikeVisitInterface) {
            $this->after_visit_classlikes[] = $class;
        }

        if ($class instanceof AfterCodebasePopulatedInterface) {
            $this->after_codebase_populated[] = $class;
        }

        if ($class instanceof BeforeAddIssueInterface) {
            $this->before_add_issue[] = $class;
        }

        if ($class instanceof AfterAnalysisInterface) {
            $this->after_analysis[] = $class;
        }

        if ($class instanceof AfterFileAnalysisInterface) {
            $this->after_file_checks[] = $class;
        }

        if ($class instanceof BeforeFileAnalysisInterface) {
            $this->before_file_checks[] = $class;
        }

        if ($class instanceof AfterFunctionLikeAnalysisInterface) {
            $this->after_functionlike_checks[] = $class;
        }

        if ($class instanceof AddTaintsInterface) {
            $this->add_taints_checks[] = $class;
        }

        if ($class instanceof RemoveTaintsInterface) {
            $this->remove_taints_checks[] = $class;
        }

        if ($class instanceof ClassFilePathProviderInterface) {
            $this->file_path_provider_interface[] = $class;
        }
    }

    /**
     * @psalm-mutation-free
     */
    public function hasAfterMethodCallAnalysisHandlers(): bool
    {
        return count($this->after_method_checks) > 0;
    }

    public function dispatchAfterMethodCallAnalysis(AfterMethodCallAnalysisEvent $event): void
    {
        foreach ($this->after_method_checks as $handler) {
            $handler->afterMethodCallAnalysis($event);
        }
    }

    public function dispatchAfterFunctionCallAnalysis(AfterFunctionCallAnalysisEvent $event): void
    {
        foreach ($this->after_function_checks as $handler) {
            $handler->afterFunctionCallAnalysis($event);
        }
    }

    public function dispatchAfterEveryFunctionCallAnalysis(AfterEveryFunctionCallAnalysisEvent $event): void
    {
        foreach ($this->after_every_function_checks as $handler) {
            $handler->afterEveryFunctionCallAnalysis($event);
        }
    }

    public function dispatchBeforeExpressionAnalysis(BeforeExpressionAnalysisEvent $event): ?bool
    {
        foreach ($this->before_expression_checks as $handler) {
            if ($handler->beforeExpressionAnalysis($event) === false) {
                return false;
            }
        }

        return null;
    }

    public function dispatchAfterExpressionAnalysis(AfterExpressionAnalysisEvent $event): ?bool
    {
        foreach ($this->after_expression_checks as $handler) {
            if ($handler->afterExpressionAnalysis($event) === false) {
                return false;
            }
        }

        return null;
    }

    public function dispatchBeforeStatementAnalysis(BeforeStatementAnalysisEvent $event): ?bool
    {
        foreach ($this->before_statement_checks as $handler) {
            if ($handler->beforeStatementAnalysis($event) === false) {
                return false;
            }
        }
        return null;
    }

    public function dispatchAfterStatementAnalysis(AfterStatementAnalysisEvent $event): ?bool
    {
        foreach ($this->after_statement_checks as $handler) {
            if ($handler->afterStatementAnalysis($event) === false) {
                return false;
            }
        }

        return null;
    }

    public function dispatchStringInterpreter(StringInterpreterEvent $event): ?TLiteralString
    {
        foreach ($this->string_interpreters as $handler) {
            if ($type = $handler->getTypeFromValue($event)) {
                return $type;
            }
        }

        return null;
    }

    public function dispatchAfterClassLikeExistenceCheck(AfterClassLikeExistenceCheckEvent $event): void
    {
        foreach ($this->after_classlike_exists_checks as $handler) {
            $handler->afterClassLikeExistenceCheck($event);
        }
    }

    public function dispatchAfterClassLikeAnalysis(AfterClassLikeAnalysisEvent $event): ?bool
    {
        foreach ($this->after_classlike_checks as $handler) {
            if ($handler->afterStatementAnalysis($event) === false) {
                return false;
            }
        }

        return null;
    }

    /**
     * @psalm-mutation-free
     */
    public function hasAfterClassLikeVisitHandlers(): bool
    {
        return count($this->after_visit_classlikes) > 0;
    }

    public function dispatchAfterClassLikeVisit(AfterClassLikeVisitEvent $event): void
    {
        foreach ($this->after_visit_classlikes as $handler) {
            $handler->afterClassLikeVisit($event);
        }
    }

    public function dispatchAfterCodebasePopulated(AfterCodebasePopulatedEvent $event): void
    {
        foreach ($this->after_codebase_populated as $handler) {
            $handler->afterCodebasePopulated($event);
        }
    }

    public function dispatchBeforeAddIssue(BeforeAddIssueEvent $event): ?bool
    {
        foreach ($this->before_add_issue as $handler) {
            $result = $handler->beforeAddIssue($event);
            if (is_bool($result)) {
                return $result;
            }
        }
        return null;
    }

    public function dispatchAfterAnalysis(AfterAnalysisEvent $event): void
    {
        foreach ($this->after_analysis as $handler) {
            $handler->afterAnalysis($event);
        }
    }

    public function dispatchAfterFileAnalysis(AfterFileAnalysisEvent $event): void
    {
        foreach ($this->after_file_checks as $handler) {
            $handler->afterAnalyzeFile($event);
        }
    }

    public function dispatchBeforeFileAnalysis(BeforeFileAnalysisEvent $event): void
    {
        foreach ($this->before_file_checks as $handler) {
            $handler->beforeAnalyzeFile($event);
        }
    }

    public function dispatchAfterFunctionLikeAnalysis(AfterFunctionLikeAnalysisEvent $event): ?bool
    {
        foreach ($this->after_functionlike_checks as $handler) {
            if ($handler->afterStatementAnalysis($event) === false) {
                return false;
            }
        }

        return null;
    }

    public function dispatchAddTaints(AddRemoveTaintsEvent $event): int
    {
        $added_taints = 0;

        foreach ($this->add_taints_checks as $handler) {
            $added_taints |= $handler->addTaints($event);
        }

        return $added_taints;
    }

    public function dispatchRemoveTaints(AddRemoveTaintsEvent $event): int
    {
        $removed_taints = 0;

        foreach ($this->remove_taints_checks as $handler) {
            $removed_taints |= $handler->removeTaints($event);
        }

        return $removed_taints;
    }
}

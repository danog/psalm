<?php

declare(strict_types=1);

namespace Psalm\Internal\Provider;

use Closure;
use PhpParser;
use Psalm\CodeLocation;
use Psalm\Context;
use Psalm\Internal\Provider\ReturnTypeProvider\ArrayChunkReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArrayColumnReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArrayCombineReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArrayFillKeysReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArrayFillReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArrayFilterReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArrayMapReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArrayMergeReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArrayPadReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArrayPointerAdjustmentReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArrayPopReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArrayRandReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArrayReduceReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArrayReverseReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArraySliceReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ArraySpliceReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\BasenameReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\DateReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\DirnameReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\FilterInputReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\FilterVarReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\FirstArgStringReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\GetClassMethodsReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\GetObjectVarsReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\HexdecReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\InArrayReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\IteratorToArrayReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\MbInternalEncodingReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\MinMaxReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\MktimeReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\ParseUrlReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\PowReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\RandReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\RoundReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\SprintfReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\StrReplaceReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\StrTrReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\TriggerErrorReturnTypeProvider;
use Psalm\Internal\Provider\ReturnTypeProvider\VersionCompareReturnTypeProvider;
use Psalm\Plugin\EventHandler\Event\FunctionReturnTypeProviderEvent;
use Psalm\Plugin\EventHandler\FunctionReturnTypeProviderInterface;
use Psalm\StatementsSource;
use Psalm\Type\Union;

use function strtolower;

/**
 * @internal
 */
final class FunctionReturnTypeProvider
{
    /**
     * @var array<
     *   lowercase-string,
     *   list<Closure(FunctionReturnTypeProviderEvent): ?Union>
     * >
     */
    private static array $handlers = [];

    public function __construct()
    {
        self::$handlers = [];

        $this->registerClass(new ArrayChunkReturnTypeProvider());
        $this->registerClass(new ArrayColumnReturnTypeProvider());
        $this->registerClass(new ArrayCombineReturnTypeProvider());
        $this->registerClass(new ArrayFilterReturnTypeProvider());
        $this->registerClass(new ArrayMapReturnTypeProvider());
        $this->registerClass(new ArrayMergeReturnTypeProvider());
        $this->registerClass(new ArrayPadReturnTypeProvider());
        $this->registerClass(new ArrayPointerAdjustmentReturnTypeProvider());
        $this->registerClass(new ArrayPopReturnTypeProvider());
        $this->registerClass(new ArrayRandReturnTypeProvider());
        $this->registerClass(new ArrayReduceReturnTypeProvider());
        $this->registerClass(new ArraySliceReturnTypeProvider());
        $this->registerClass(new ArraySpliceReturnTypeProvider());
        $this->registerClass(new ArrayReverseReturnTypeProvider());
        $this->registerClass(new ArrayFillReturnTypeProvider());
        $this->registerClass(new ArrayFillKeysReturnTypeProvider());
        $this->registerClass(new FilterInputReturnTypeProvider());
        $this->registerClass(new FilterVarReturnTypeProvider());
        $this->registerClass(new IteratorToArrayReturnTypeProvider());
        $this->registerClass(new ParseUrlReturnTypeProvider());
        $this->registerClass(new StrReplaceReturnTypeProvider());
        $this->registerClass(new StrTrReturnTypeProvider());
        $this->registerClass(new VersionCompareReturnTypeProvider());
        $this->registerClass(new MktimeReturnTypeProvider());
        $this->registerClass(new BasenameReturnTypeProvider());
        $this->registerClass(new DirnameReturnTypeProvider());
        $this->registerClass(new GetObjectVarsReturnTypeProvider());
        $this->registerClass(new GetClassMethodsReturnTypeProvider());
        $this->registerClass(new FirstArgStringReturnTypeProvider());
        $this->registerClass(new HexdecReturnTypeProvider());
        $this->registerClass(new MinMaxReturnTypeProvider());
        $this->registerClass(new TriggerErrorReturnTypeProvider());
        $this->registerClass(new RandReturnTypeProvider());
        $this->registerClass(new InArrayReturnTypeProvider());
        $this->registerClass(new RoundReturnTypeProvider());
        $this->registerClass(new MbInternalEncodingReturnTypeProvider());
        $this->registerClass(new DateReturnTypeProvider());
        $this->registerClass(new PowReturnTypeProvider());
        $this->registerClass(new SprintfReturnTypeProvider());
    }

    /**
     * Registers a provider object (classes are never looked up by name: the program is compiled).
     */
    public function registerClass(object $class): void
    {
        if ($class instanceof FunctionReturnTypeProviderInterface) {
            $callable = $class->getFunctionReturnType(...);

            foreach ($class->getFunctionIds() as $function_id) {
                $this->registerClosure($function_id, $callable);
            }
        }
    }

    /**
     * @param lowercase-string $function_id
     * @param Closure(FunctionReturnTypeProviderEvent): ?Union $c
     * @psalm-external-mutation-free
     */
    public function registerClosure(string $function_id, Closure $c): void
    {
        self::$handlers[$function_id][] = $c;
    }

    /**
     * @psalm-external-mutation-free
     */
    public function has(string $function_id): bool
    {
        return isset(self::$handlers[strtolower($function_id)]);
    }

    /**
     * @param  non-empty-string $function_id
     */
    public function getReturnType(
        StatementsSource $statements_source,
        string $function_id,
        PhpParser\Node\Expr\FuncCall $stmt,
        Context $context,
        CodeLocation $code_location,
    ): ?Union {
        foreach (self::$handlers[strtolower($function_id)] ?? [] as $function_handler) {
            $event = new FunctionReturnTypeProviderEvent(
                $statements_source,
                $function_id,
                $stmt,
                $context,
                $code_location,
            );
            $return_type = $function_handler($event);

            if ($return_type) {
                return $return_type;
            }
        }

        return null;
    }
}

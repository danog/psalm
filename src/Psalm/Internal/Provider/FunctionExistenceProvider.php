<?php

declare(strict_types=1);

namespace Psalm\Internal\Provider;

use Closure;
use Psalm\Plugin\EventHandler\Event\FunctionExistenceProviderEvent;
use Psalm\Plugin\EventHandler\FunctionExistenceProviderInterface;
use Psalm\StatementsSource;

use function strtolower;

/**
 * @internal
 */
final class FunctionExistenceProvider
{
    /**
     * @var array<
     *   lowercase-string,
     *   list<Closure(FunctionExistenceProviderEvent): ?bool>
     * >
     */
    private static array $handlers = [];

    /**
     * @psalm-mutation-free
     */
    public function __construct()
    {
        self::$handlers = [];
    }

    /**
     * Registers a provider object (classes are never looked up by name: the program is compiled).
     */
    public function registerClass(object $class): void
    {
        if ($class instanceof FunctionExistenceProviderInterface) {
            $callable = $class->doesFunctionExist(...);

            foreach ($class->getFunctionIds() as $function_id) {
                $this->registerClosure($function_id, $callable);
            }
        }
    }

    /**
     * @param lowercase-string $function_id
     * @param Closure(FunctionExistenceProviderEvent): ?bool $c
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

    public function doesFunctionExist(
        StatementsSource $statements_source,
        string $function_id,
    ): ?bool {
        foreach (self::$handlers[strtolower($function_id)] ?? [] as $function_handler) {
            $event = new FunctionExistenceProviderEvent(
                $statements_source,
                $function_id,
            );
            $function_exists = $function_handler($event);

            if ($function_exists !== null) {
                return $function_exists;
            }
        }

        return null;
    }
}

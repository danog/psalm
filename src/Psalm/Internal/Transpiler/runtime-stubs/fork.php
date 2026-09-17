<?php

/**
 * Typed stand-ins for the parallel (forked) analysis machinery. The transpiled program is single-process:
 * the pool paths are never taken (threads = 1), so these keep the callers statically typed and fail loudly
 * if ever reached.
 */

namespace Amp {
    interface Cancellation
    {
        public function isRequested(): bool;

        public function throwIfRequested(): void;
    }

    /** @template T */
    final class Future
    {
        /** @return T */
        public function await(?Cancellation $cancellation = null)
        {
            throw new \RuntimeException('parallel execution is not available');
        }

        /**
         * @template Tv
         * @param iterable<Future<Tv>> $futures
         * @return iterable<int, Future<Tv>>
         */
        public static function iterate(iterable $futures, ?Cancellation $cancellation = null): iterable
        {
            return [];
        }
    }
}

namespace Amp\Future {
    /**
     * @template T
     * @param iterable<Future<T>> $futures
     * @return array<int, T>
     */
    function await(iterable $futures, ?\Amp\Cancellation $cancellation = null): array
    {
        return [];
    }
}

namespace Amp\Sync {
    /**
     * Messages are arrays or strings (psalm's taint and progress messages).
     *
     * @template-covariant TReceive of array|string
     * @template TSend of array|string
     */
    interface Channel
    {
        /** @return TReceive */
        public function receive(?\Amp\Cancellation $cancellation = null): array|string;

        /** @param TSend $data */
        public function send(array|string $data): void;

        public function close(): void;

        public function isClosed(): bool;
    }
}

namespace Amp\Parallel\Worker {
    /**
     * @template-covariant TResult
     * @template TReceive
     * @template TSend
     */
    interface Task
    {
        /**
         * @param \Amp\Sync\Channel<TReceive, TSend> $channel
         * @return TResult
         */
        public function run(\Amp\Sync\Channel $channel, \Amp\Cancellation $cancellation): mixed;
    }
}

namespace Psalm\Internal\Fork {
    use Amp\Cancellation;
    use Amp\Future;
    use Amp\Parallel\Worker\Task;
    use Amp\Sync\Channel;
    use Closure;
    use Psalm\Progress\Progress;
    use RuntimeException;

    /** The xdebug-handler based restarter: the port never re-executes itself. */
    final class PsalmRestarter
    {
        public bool $enableJit = false;

        public function __construct(string $envPrefix)
        {
        }

        public function disableExtension(string $disabled_extension): void
        {
        }

        /** @param list<string> $disable_extensions */
        public function disableExtensions(array $disable_extensions): void
        {
        }

        public function check(): bool
        {
            return false;
        }
    }

    final class Pool
    {
        /** @param int<2, max> $threads */
        public function __construct(
            public readonly int $threads,
            private readonly float $timeLimit,
            private readonly Progress $progress,
        ) {
        }

        /**
         * @template TResult
         * @param list<string> $process_task_data_iterator
         * @param Closure(string): Task<TResult, void, void> $task_factory
         * @param null|Closure(TResult):void $task_done_closure
         * @param null|Closure(mixed):mixed $message_handler
         */
        public function run(
            array $process_task_data_iterator,
            Closure $task_factory,
            ?Closure $task_done_closure = null,
            ?Closure $message_handler = null,
        ): void {
            throw new RuntimeException('parallel execution is not available');
        }

        /**
         * @template T
         * @param Task<T, void, void> $task
         * @return array<int, Future<T>>
         */
        public function runAll(Task $task): array
        {
            throw new RuntimeException('parallel execution is not available');
        }
    }

    /** @implements Task<int, void, void> */
    final class AnalyzerTask implements Task
    {
        public function __construct(private string $file)
        {
        }

        public function run(Channel $channel, Cancellation $cancellation): int
        {
            throw new RuntimeException('parallel execution is not available: ' . $this->file);
        }
    }

    /** @implements Task<null, array{id: int|null, count: int}, string> */
    final class ScannerTask implements Task
    {
        public function __construct(private string $file)
        {
        }

        /** @return null */
        public function run(Channel $channel, Cancellation $cancellation): mixed
        {
            throw new RuntimeException('parallel execution is not available: ' . $this->file);
        }
    }

    /** @implements Task<null, void, void> */
    final class InitAnalyzerTask implements Task
    {
        /** @return null */
        public function run(Channel $channel, Cancellation $cancellation): mixed
        {
            throw new RuntimeException('parallel execution is not available');
        }
    }

    /** @implements Task<null, void, void> */
    final class InitScannerTask implements Task
    {
        /** @return null */
        public function run(Channel $channel, Cancellation $cancellation): mixed
        {
            throw new RuntimeException('parallel execution is not available');
        }
    }

    /**
     * @psalm-import-type WorkerData from \Psalm\Internal\Codebase\Analyzer
     * @implements Task<WorkerData, void, void>
     */
    final class ShutdownAnalyzerTask implements Task
    {
        /** @return WorkerData */
        public function run(Channel $channel, Cancellation $cancellation): mixed
        {
            throw new RuntimeException('parallel execution is not available');
        }
    }

    /**
     * @psalm-import-type PoolData from \Psalm\Internal\Codebase\Scanner
     * @implements Task<PoolData, void, void>
     */
    final class ShutdownScannerTask implements Task
    {
        /** @return PoolData */
        public function run(Channel $channel, Cancellation $cancellation): mixed
        {
            throw new RuntimeException('parallel execution is not available');
        }
    }
}

namespace Amp\PHPUnit {

/**
 * The Amp async test base: the compiled tests run synchronously (no event loop), so it is PHPUnit's TestCase
 * with the fixture hooks the Psalm tests override.
 */
abstract class AsyncTestCase extends \PHPUnit\Framework\TestCase
{
    public static function setUpBeforeClass(): void
    {
        parent::setUpBeforeClass();
    }

    public function setUp(): void
    {
        parent::setUp();
    }

    public function tearDown(): void
    {
        parent::tearDown();
    }

    protected function setTimeout(float $seconds): void
    {
    }
}
}

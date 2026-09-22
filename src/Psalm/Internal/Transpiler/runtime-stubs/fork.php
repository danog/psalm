<?php

/**
 * The parallel analysis machinery of the compiled program: amphp's futures, channels and tasks as the
 * typed shapes Psalm's code talks to, and a worker pool on fork() in place of amphp's process pool.
 */

namespace Amp {
    interface Cancellation
    {
        public function isRequested(): bool;
        public function throwIfRequested(): void;
    }
    /**
     * A worker's result, decoded when awaited. Holds the encoded bytes a worker sent, or an already
     * decoded value; decoding happens in await(), the one place whose declared return type names T.
     *
     * @template T
     */
    final class Future
    {
        /** @var bool a value rather than bytes */
        private bool $resolved;

        public function __construct(private string $bytes, private bool $decoded_null = false)
        {
            $this->resolved = $decoded_null;
        }

        /** A future that resolves to what the bytes encode. */
        public static function ofBytes(string $bytes): Future
        {
            return new Future($bytes);
        }

        /** @return T */
        public function await(?Cancellation $cancellation = null)
        {
            return __rt_decode($this->bytes);
        }

        /**
         * @template Tv
         * @param iterable<Future<Tv>> $futures
         * @return iterable<int, Future<Tv>>
         */
        public static function iterate(iterable $futures, ?Cancellation $cancellation = null): iterable
        {
            $out = [];
            foreach ($futures as $f) {
                $out[] = $f;
            }
            return $out;
        }
    }
    final class NullCancellation implements Cancellation
    {
        public function isRequested(): bool
        {
            return false;
        }
        public function throwIfRequested(): void
        {
        }
    }
}
namespace Amp\Future {
    /**
     * @template T
     * @param iterable<\Amp\Future<T>> $futures
     * @return array<int, T>
     */
    function await(iterable $futures, ?\Amp\Cancellation $cancellation = null): array
    {
        $out = [];
        foreach ($futures as $f) {
            $out[] = $f->await();
        }
        return $out;
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
        /** @return array{id: int|null, count: int}|string */
        public function receive(?\Amp\Cancellation $cancellation = null): array|string;
        /** @param array{id: int|null, count: int}|string $data */
        public function send(array|string $data): void;
        public function close(): void;
        public function isClosed(): bool;
    }
    /**
     * The channel a forked worker sees: messages to the parent are dropped (progress is reported
     * once the worker finishes), and nothing is ever received.
     *
     * @implements Channel<array|string, array|string>
     */
    final class WorkerChannel implements Channel
    {
        public function receive(?\Amp\Cancellation $cancellation = null): array|string
        {
            throw new \RuntimeException('a forked worker receives no messages');
        }
        public function send(array|string $data): void
        {
        }
        public function close(): void
        {
        }
        public function isClosed(): bool
        {
            return false;
        }
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
    use Amp\NullCancellation;
    use Amp\Parallel\Worker\Task;
    use Amp\Sync\Channel;
    use Amp\Sync\WorkerChannel;
    use Closure;
    use Psalm\Internal\Analyzer\ProjectAnalyzer;
    use Psalm\Progress\Progress;
    use Psalm\Progress\VoidProgress;
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

    /**
     * Psalm's worker pool, on fork(): the same three calls Psalm makes against the amphp pool -- an
     * init task on every worker, one task per file, a shutdown task returning each worker's data --
     * but run once the shutdown task is known: then each forked worker (which inherits the scanned
     * codebase copy-on-write) runs init, its share of the files and shutdown, and sends its encoded
     * result to the parent, which hands the results back as futures.
     */
    final class Pool
    {
        /** @var Task<mixed, mixed, mixed>|null */
        private ?Task $init_task = null;
        /** @var list<string> */
        private array $files = [];
        /** @var Closure(string): Task<mixed, mixed, mixed>|null */
        private ?Closure $task_factory = null;
        /** @var Closure(int): void|null */
        private ?Closure $task_done = null;

        /** @param int<2, max> $threads */
        public function __construct(
            public readonly int $threads,
            private readonly float $timeLimit,
            private readonly Progress $progress,
        ) {
        }

        /**
         * @param list<string> $process_task_data_iterator
         * @param Closure(string): Task<mixed, mixed, mixed> $task_factory
         * @param null|Closure(int): void $task_done_closure the count of finished items
         * @param null|Closure(string): array{id: int|null, count: int} $message_handler
         */
        public function run(
            array $process_task_data_iterator,
            Closure $task_factory,
            ?Closure $task_done_closure = null,
            ?Closure $message_handler = null,
        ): void {
            $this->files = array_values($process_task_data_iterator);
            $this->task_factory = $task_factory;
            $this->task_done = $task_done_closure;
        }

        /**
         * @template T
         * @param Task<T, mixed, mixed> $task
         * @return array<int, Future<T>>
         */
        public function runAll(Task $task): array
        {
            $futures = [];
            if ($this->task_factory === null) {
                // the init task: it runs in every worker once they start; its result is null
                $this->init_task = $task;
                for ($i = 0; $i < $this->threads; $i++) {
                    $futures[] = Future::ofBytes(__rt_encode(null));
                }
                return $futures;
            }
            $workers = max(1, min($this->threads, count($this->files)));
            $index = __rt_fork_workers($workers);
            if ($index >= 0) {
                $this->work($index, $workers, $task);
            }
            $payloads = __rt_collect_workers();
            $this->files = [];
            $this->task_factory = null;
            if ($this->task_done !== null) {
                foreach ($this->files as $_) {
                    ($this->task_done)(0);
                }
            }
            foreach ($payloads as $bytes) {
                $futures[] = Future::ofBytes($bytes);
            }
            return $futures;
        }

        /**
         * A forked worker's whole life: init, every file that is its share, shutdown, send, exit.
         *
         * @param Task<mixed, mixed, mixed> $shutdown_task
         */
        private function work(int $index, int $workers, Task $shutdown_task): never
        {
            ProjectAnalyzer::getInstance()->progress = new VoidProgress();
            $channel = new WorkerChannel();
            $cancellation = new NullCancellation();
            if ($this->init_task !== null) {
                $this->init_task->run($channel, $cancellation);
            }
            $factory = $this->task_factory;
            if ($factory !== null) {
                foreach ($this->files as $i => $file) {
                    if ($i % $workers === $index) {
                        $factory($file)->run($channel, $cancellation);
                    }
                }
            }
            __rt_worker_exit(__rt_encode($shutdown_task->run($channel, $cancellation)));
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

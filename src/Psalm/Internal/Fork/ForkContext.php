<?php

declare(strict_types=1);

namespace Psalm\Internal\Fork;

use Amp\ByteStream\StreamChannel;
use Amp\Cancellation;
use Amp\Parallel\Context\ContextException;
use Amp\Parallel\Context\Internal\AbstractContext;
use Amp\Parallel\Context\Internal\ContextChannel;
use Amp\Parallel\Context\Internal\ExitFailure;
use Amp\Parallel\Context\Internal\ExitSuccess;
use Amp\Parallel\Ipc\IpcHub;
use Amp\Serialization\NativeSerializer;
use Amp\Serialization\SerializationException;
use Amp\TimeoutCancellation;
use Override;
use Psalm\Internal\TypePhp\Dynamic;
use Revolt\EventLoop;
use RuntimeException;
use Throwable;

use function Amp\Parallel\Ipc\connect;
use function define;
use function extension_loaded;
use function fprintf;
use function fwrite;
use function pcntl_fork;
use function pcntl_waitpid;
use function pcntl_wexitstatus;
use function pcntl_wifexited;
use function pcntl_wifsignaled;
use function pcntl_wifstopped;
use function pcntl_wstopsig;
use function pcntl_wtermsig;
use function posix_get_last_error;
use function posix_kill;
use function posix_strerror;

use const PHP_EOL;
use const STDERR;
use const WNOHANG;

/**
 * @internal
 * @template-covariant TResult
 * @template-covariant TReceive
 * @template TSend
 * @extends AbstractContext<TResult, TReceive, TSend>
 */
final class ForkContext extends AbstractContext
{
    private const DEFAULT_START_TIMEOUT = 5;

    /**
     * @param string|non-empty-list<string> $argv Path to PHP script or array with first element as path and
     *     following elements options to the PHP script (e.g.: ['bin/worker.php', 'Option1Value', 'Option2Value']).
     * @param positive-int $childConnectTimeout Number of seconds the child will attempt to connect to the parent
     *      before failing.
     * @throws ContextException If starting the process fails.
     */
    public static function start(
        string|array $argv,
        IpcHub $ipcHub,
        ?Cancellation $cancellation = null,
        int $childConnectTimeout = self::DEFAULT_START_TIMEOUT,
    ): self {
        $serializer = extension_loaded('igbinary')
            ? new IgbinarySerializer
            : new NativeSerializer;

        $key = $ipcHub->generateKey();

        // Fork
        if (($pid = pcntl_fork()) < 0) {
            throw new RuntimeException(posix_strerror(posix_get_last_error()));
        }

        // Parent
        if ($pid > 0) {
            try {
                $socket = $ipcHub->accept($key, $cancellation);
                $ipcChannel = new StreamChannel($socket, $socket, $serializer);
        
                $socket = $ipcHub->accept($key, $cancellation);
                $resultChannel = new StreamChannel($socket, $socket, $serializer);
            } catch (Throwable $exception) {
                $cancellation?->throwIfRequested();
        
                throw new ContextException("Starting the process failed", 0, $exception);
            }
        
            return new self($pid, $ipcChannel, $resultChannel);
        }

        // Child
        define("AMP_CONTEXT", "parallel");

        $connectCancellation = new TimeoutCancellation((float) $childConnectTimeout);
        $uri = $ipcHub->getUri();

        try {
            $socket = connect($uri, $key, $connectCancellation);
            $ipcChannel = new StreamChannel($socket, $socket, $serializer);

            $socket = connect($uri, $key, $connectCancellation);
            $resultChannel = new StreamChannel($socket, $socket, $serializer);
        } catch (Throwable $exception) {
            fwrite(STDERR, 'Could not connect the worker to the parent: ' . $exception->getMessage() . PHP_EOL);
            exit(1);
        }

        try {
            /** @psalm-suppress InternalClass, InternalMethod */
            $result = Dynamic::any(new ExitSuccess(TaskRunner::run(new ContextChannel($ipcChannel))));
        } catch (Throwable $exception) {
            $result = new ExitFailure($exception);
        }

        try {
            try {
                $resultChannel->send($result);
            } catch (SerializationException $exception) {
                // Serializing the result failed. Send the reason why.
                $resultChannel->send(new ExitFailure($exception));
            }
        } catch (Throwable $exception) {
            fprintf(
                STDERR,
                "Could not send result to parent: '%s'; be sure to shutdown the child before ending the parent".PHP_EOL,
                $exception->getMessage(),
            );
        }

        if (EventLoop::getDriver()->isRunning()) {
            // Forked from inside an event loop callback (the worker pool creates
            // workers asynchronously): the inherited loop is still running in
            // this process, so it cannot be started again; the result has been
            // sent, nothing else is pending.
            exit(0);
        }

        EventLoop::run();

        fwrite(STDERR, "ERROR IN WORKER: Unreachable!".PHP_EOL);
        exit(1);
    }

    private ?int $exited = null;

    /**
     * @param StreamChannel<TReceive, TSend> $ipcChannel
     */
    private function __construct(
        private readonly int $pid,
        StreamChannel $ipcChannel,
        StreamChannel $resultChannel,
    ) {
        parent::__construct($ipcChannel, $resultChannel);
    }

    public function __destruct()
    {
        $this->close();
    }

    #[Override]
    public function receive(?Cancellation $cancellation = null): mixed
    {
        $this->checkExit();

        return parent::receive($cancellation);
    }

    #[Override]
    public function send(mixed $data): void
    {
        $this->checkExit();

        parent::send($data);
    }

    /**
     * @psalm-external-mutation-free
     */
    private function checkExit(bool $wait = false): ?int
    {
        if ($this->exited === null) {
            if (pcntl_waitpid($this->pid, $status, $wait ? 0 : WNOHANG) === 0) {
                return null;
            }

            $signal = -1;
            if (pcntl_wifsignaled($status)) {
                $signal = pcntl_wtermsig($status);
            } elseif (pcntl_wifexited($status)) {
                $signal = pcntl_wexitstatus($status) - 128;
            } elseif (pcntl_wifstopped($status)) {
                $signal = pcntl_wstopsig($status);
            }
            $this->exited = $signal;
        }

        if (!$this->weKilled && $this->exited > 0) {
            $exit_signal = $this->exited;
            if ($exit_signal === 11) {
                $signal_description = "11: THIS IS A PHP BUG, please report this to https://github.com/vimeo/psalm/issues".
                    " AND to https://github.com/php/php-src/issues";
            } elseif ($exit_signal === 9) {
                $signal_description = "9: the process was likely killed by the OOM killer, try increasing the swap space ".
                    "or use the arrayCache=\"false\" config to reduce memory usage";
            } else {
                $signal_description = (string) $exit_signal;
            }
            throw new ContextException("Worker exited due to signal $signal_description!");
        }

        return $this->exited;
    }

    private bool $weKilled = false;

    #[Override]
    public function close(): void
    {
        if ($this->checkExit() === null) {
            $this->weKilled = true;
            posix_kill($this->pid, 9);

            $this->checkExit(true);
        }

        parent::close();
    }

    #[Override]
    public function join(?Cancellation $cancellation = null): mixed
    {
        try {
            $data = $this->receiveExitResult($cancellation);
        } finally {
            $this->close();
        }

        return $data->getResult();
    }
}

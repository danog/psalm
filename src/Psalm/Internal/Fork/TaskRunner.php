<?php

declare(strict_types=1);

namespace Psalm\Internal\Fork;

use Amp\CancelledException;
use Amp\DeferredCancellation;
use Amp\Future;
use Amp\Parallel\Worker\Internal\JobCancellation;
use Amp\Parallel\Worker\Internal\JobChannel;
use Amp\Parallel\Worker\Internal\JobMessage;
use Amp\Parallel\Worker\Internal\TaskCancelled;
use Amp\Parallel\Worker\Internal\TaskFailure;
use Amp\Parallel\Worker\Internal\TaskSubmission;
use Amp\Parallel\Worker\Internal\TaskSuccess;
use Amp\Pipeline\Queue;
use Amp\Serialization\SerializationException;
use Amp\Sync\Channel;
use Error;
use Revolt\EventLoop;
use Throwable;

use function constant;
use function define;
use function defined;
use function get_debug_type;

/**
 * The worker loop of a forked Psalm process: a copy of amphp/parallel's
 * task-runner.php script, so that no script has to be loaded at runtime.
 *
 * @internal
 * @psalm-suppress InternalClass, InternalMethod, MixedAssignment, MixedArgument
 */
final class TaskRunner
{
    public static function run(Channel $channel): int
    {
        if (!defined('AMP_WORKER')) {
            define('AMP_WORKER', constant('AMP_CONTEXT'));
        }

        /** @var array<string, DeferredCancellation> $cancellation_sources */
        $cancellation_sources = [];

        /** @var array<string, Queue> $queues */
        $queues = [];

        while ($data = $channel->receive()) {
            if ($data instanceof TaskSubmission) {
                $id = $data->getId();

                $cancellation_sources[$id] = $source = new DeferredCancellation;
                $queues[$id] = $queue = new Queue();

                $job_channel = new JobChannel($id, $channel, $queue->iterate());

                EventLoop::queue(static function () use (
                    &$cancellation_sources,
                    &$queues,
                    $data,
                    $id,
                    $source,
                    $queue,
                    $job_channel,
                    $channel,
                ): void {
                    try {
                        $result = $data->getTask()->run($job_channel, $source->getCancellation());

                        if ($result instanceof Future) {
                            $result = $result->await($source->getCancellation());
                        }

                        $result = new TaskSuccess($data->getId(), $result);
                    } catch (Throwable $exception) {
                        if ($exception instanceof CancelledException && $source->isCancelled()) {
                            $result = new TaskCancelled($id, $exception);
                        } else {
                            $result = new TaskFailure($id, $exception);
                        }
                    } finally {
                        $queue->complete();
                        unset($cancellation_sources[$id], $queues[$id]);
                    }

                    try {
                        $channel->send($result);
                    } catch (SerializationException $exception) {
                        // Could not serialize task result.
                        $channel->send(new TaskFailure($id, $exception));
                    }
                });
                continue;
            }

            if ($data instanceof JobMessage) {
                ($queues[$data->getId()] ?? null)?->pushAsync($data->getMessage())->ignore();
                continue;
            }

            if ($data instanceof JobCancellation) {
                ($cancellation_sources[$data->getId()] ?? null)?->cancel();
                continue;
            }

            throw new Error('Invalid value ' . get_debug_type($data) . ' received in ' . __METHOD__);
        }

        return 0;
    }
}

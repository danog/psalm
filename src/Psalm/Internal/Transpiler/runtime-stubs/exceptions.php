<?php

declare(strict_types=1);

/**
 * PHP implementations of the builtin exception hierarchy. Transpiled to Rust like project code.
 */

interface Stringable
{
    public function __toString(): string;
}

interface Throwable extends Stringable
{
    public function getMessage(): string;

    public function getCode(): int;

    public function getFile(): string;

    public function getLine(): int;

    /** @return list<array<string, mixed>> */
    public function getTrace(): array;

    public function getTraceAsString(): string;

    public function getPrevious(): ?Throwable;
}

class Exception implements Throwable
{
    protected string $message = '';

    protected int $code = 0;

    protected string $file = '';

    protected int $line = 0;

    protected ?Throwable $previous = null;

    public function __construct(string $message = '', int $code = 0, ?Throwable $previous = null)
    {
        $this->message = $message;
        $this->code = $code;
        $this->previous = $previous;
    }

    public function getMessage(): string
    {
        return $this->message;
    }

    public function getCode(): int
    {
        return $this->code;
    }

    public function getFile(): string
    {
        return $this->file;
    }

    public function getLine(): int
    {
        return $this->line;
    }

    /** @return list<array<string, mixed>> */
    public function getTrace(): array
    {
        return [];
    }

    public function getTraceAsString(): string
    {
        return '#0 {main}';
    }

    public function getPrevious(): ?Throwable
    {
        return $this->previous;
    }

    public function __toString(): string
    {
        return static::class . ': ' . $this->message;
    }
}

class Error implements Throwable
{
    protected string $message = '';

    protected int $code = 0;

    protected string $file = '';

    protected int $line = 0;

    protected ?Throwable $previous = null;

    public function __construct(string $message = '', int $code = 0, ?Throwable $previous = null)
    {
        $this->message = $message;
        $this->code = $code;
        $this->previous = $previous;
    }

    public function getMessage(): string
    {
        return $this->message;
    }

    public function getCode(): int
    {
        return $this->code;
    }

    public function getFile(): string
    {
        return $this->file;
    }

    public function getLine(): int
    {
        return $this->line;
    }

    /** @return list<array<string, mixed>> */
    public function getTrace(): array
    {
        return [];
    }

    public function getTraceAsString(): string
    {
        return '#0 {main}';
    }

    public function getPrevious(): ?Throwable
    {
        return $this->previous;
    }

    public function __toString(): string
    {
        return static::class . ': ' . $this->message;
    }
}

class ErrorException extends Exception
{
    protected int $severity = 1;

    public function __construct(
        string $message = '',
        int $code = 0,
        int $severity = 1,
        ?string $filename = null,
        ?int $line = null,
        ?Throwable $previous = null,
    ) {
        parent::__construct($message, $code, $previous);
        $this->severity = $severity;
        if ($filename !== null) {
            $this->file = $filename;
        }
        if ($line !== null) {
            $this->line = $line;
        }
    }

    public function getSeverity(): int
    {
        return $this->severity;
    }
}

class TypeError extends Error
{
}

class ValueError extends Error
{
}

class ArithmeticError extends Error
{
}

class DivisionByZeroError extends ArithmeticError
{
}

class ArgumentCountError extends TypeError
{
}

class AssertionError extends Error
{
}

class UnhandledMatchError extends Error
{
}

class LogicException extends Exception
{
}

class BadFunctionCallException extends LogicException
{
}

class BadMethodCallException extends BadFunctionCallException
{
}

class DomainException extends LogicException
{
}

class InvalidArgumentException extends LogicException
{
}

class LengthException extends LogicException
{
}

class OutOfRangeException extends LogicException
{
}

class RuntimeException extends Exception
{
}

class ReflectionException extends Exception
{
}

class OutOfBoundsException extends RuntimeException
{
}

class OverflowException extends RuntimeException
{
}

class RangeException extends RuntimeException
{
}

class UnderflowException extends RuntimeException
{
}

class UnexpectedValueException extends RuntimeException
{
}

class JsonException extends Exception
{
}

/** Raised by exit()/die(); carries the exit status. */
final class PhpExitException extends Exception
{
    public function __construct(public int $status = 0)
    {
        parent::__construct('exit');
    }
}

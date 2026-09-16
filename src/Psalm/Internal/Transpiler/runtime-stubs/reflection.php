<?php

declare(strict_types=1);

/**
 * Minimal ReflectionClass used by transpiled code (php-parser's JsonDecoder etc.).
 */
/**
 * Minimal ReflectionFunction: internal functions are the runtime's builtins, user functions the compiled ones.
 */
class ReflectionFunction
{
    public string $name;

    /** @param \Closure|string $function */
    public function __construct($function)
    {
        if (!is_string($function)) {
            throw new ReflectionException('Closures cannot be reflected in a compiled program');
        }
        if (!function_exists($function)) {
            throw new ReflectionException('Function ' . $function . '() does not exist');
        }
        $this->name = ltrim($function, '\\');
    }

    public function getName(): string
    {
        return $this->name;
    }

    public function isInternal(): bool
    {
        return __rt_function_is_builtin($this->name);
    }

    public function isUserDefined(): bool
    {
        return !$this->isInternal();
    }

    /** @return string|false */
    public function getFileName()
    {
        return false;
    }

    public function getNumberOfParameters(): int
    {
        return 0;
    }

    /** @return list<never> */
    public function getParameters(): array
    {
        return [];
    }

    /** No return type information exists for functions in a compiled program. */
    public function getReturnType(): ?ReflectionType
    {
        return null;
    }

    public function hasTentativeReturnType(): bool
    {
        return false;
    }

    public function getTentativeReturnType(): ?ReflectionType
    {
        return null;
    }

    public function hasReturnType(): bool
    {
        return false;
    }
}

abstract class ReflectionType
{
    public function allowsNull(): bool
    {
        return true;
    }

    abstract public function __toString(): string;
}

final class ReflectionNamedType extends ReflectionType
{
    public function __construct(private readonly string $name)
    {
    }

    public function getName(): string
    {
        return $this->name;
    }

    public function isBuiltin(): bool
    {
        return true;
    }

    #[\Override]
    public function __toString(): string
    {
        return $this->name;
    }
}

final class ReflectionUnionType extends ReflectionType
{
    /** @param list<ReflectionNamedType> $types */
    public function __construct(private readonly array $types)
    {
    }

    /** @return list<ReflectionNamedType> */
    public function getTypes(): array
    {
        return $this->types;
    }

    #[\Override]
    public function __toString(): string
    {
        return implode('|', array_map(static fn(ReflectionNamedType $t): string => $t->getName(), $this->types));
    }
}

class ReflectionClass
{
    public string $name;

    /** @param object|class-string $objectOrClass */
    public function __construct(object|string $objectOrClass)
    {
        $this->name = is_object($objectOrClass) ? get_class($objectOrClass) : $objectOrClass;
    }

    public function getName(): string
    {
        return $this->name;
    }

    public function getShortName(): string
    {
        $pos = strrpos($this->name, '\\');
        return $pos === false ? $this->name : substr($this->name, $pos + 1);
    }

    public function isSubclassOf(string $class): bool
    {
        return is_subclass_of($this->name, $class);
    }

    public function implementsInterface(string $interface): bool
    {
        return is_a($this->name, $interface, true);
    }

    public function newInstanceWithoutConstructor(): object
    {
        throw new ReflectionException('Instantiation by class name is not supported in a compiled program');
    }

    /**
     * Interfaces of builtin classes are not modelled by the compiled program.
     *
     * @return array<string, ReflectionClass>
     */
    public function getInterfaces(): array
    {
        return [];
    }

    public function isInstantiable(): bool
    {
        return class_exists($this->name);
    }

    public function isInterface(): bool
    {
        return interface_exists($this->name);
    }

    public function isTrait(): bool
    {
        return __rt_class_is_trait($this->name);
    }

    public function isAbstract(): bool
    {
        return !class_exists($this->name) || !$this->isInstantiable();
    }

    /** Classes compiled from the project have a file; builtin ones don't. */
    public function isUserDefined(): bool
    {
        return __rt_class_file($this->name) !== null;
    }

    public function isInternal(): bool
    {
        return !$this->isUserDefined();
    }

    /** @return string|false */
    /** Source positions are not available for classes in a compiled program. */
    public function getStartLine(): int|false
    {
        return false;
    }

    public function getFileName()
    {
        $file = __rt_class_file($this->name);
        return $file === null || $file === '' ? false : $file;
    }

    /** @return array<string, mixed> */
    public function getConstants(): array
    {
        return __rt_class_constants($this->name);
    }

    public function hasConstant(string $name): bool
    {
        return array_key_exists($name, __rt_class_constants($this->name));
    }

    public function getConstant(string $name): mixed
    {
        return __rt_class_constants($this->name)[$name] ?? false;
    }

    /** @return list<never> members of builtin classes are not reflectable in a compiled program */
    public function getMethods(?int $filter = null): array
    {
        return [];
    }

    /** @return list<never> */
    public function getProperties(?int $filter = null): array
    {
        return [];
    }

    /** @return list<string> */
    public function getInterfaceNames(): array
    {
        return [];
    }

    public function getParentClass(): ReflectionClass|false
    {
        $parent = get_parent_class($this->name);
        if (!$parent) {
            return false;
        }
        return new ReflectionClass($parent);
    }
}

/**
 * Reflection of members: never produced at runtime in a compiled program (reflection of internal functions is
 * guarded out and classes are compiled in), but typed so the code that would consume them stays typed.
 */
abstract class ReflectionFunctionAbstract
{
    public string $name = '';

    public function getName(): string
    {
        return $this->name;
    }

    /** @return list<ReflectionParameter> */
    public function getParameters(): array
    {
        return [];
    }

    public function getNumberOfParameters(): int
    {
        return 0;
    }

    public function getReturnType(): ?ReflectionType
    {
        return null;
    }

    public function getTentativeReturnType(): ?ReflectionType
    {
        return null;
    }

    public function hasReturnType(): bool
    {
        return false;
    }

    public function isStatic(): bool
    {
        return false;
    }
}

class ReflectionMethod extends ReflectionFunctionAbstract
{
    public const IS_STATIC = 16;
    public const IS_PUBLIC = 1;
    public const IS_PROTECTED = 2;
    public const IS_PRIVATE = 4;
    public const IS_ABSTRACT = 64;
    public const IS_FINAL = 32;

    public string $class = '';

    /** @param object|string $objectOrMethod */
    public function __construct($objectOrMethod, ?string $method = null)
    {
        throw new ReflectionException('Methods cannot be reflected in a compiled program');
    }

    public function getDeclaringClass(): ReflectionClass
    {
        return new ReflectionClass($this->class);
    }

    public function isPublic(): bool
    {
        return true;
    }

    public function isProtected(): bool
    {
        return false;
    }

    public function isPrivate(): bool
    {
        return false;
    }

    public function isAbstract(): bool
    {
        return false;
    }
}

class ReflectionParameter
{
    public string $name = '';

    /** @param string|array{0: object|string, 1: string}|object $function */
    public function __construct($function, int|string $param)
    {
        throw new ReflectionException('Parameters cannot be reflected in a compiled program');
    }

    public function getName(): string
    {
        return $this->name;
    }

    public function getType(): ?ReflectionType
    {
        return null;
    }

    public function hasType(): bool
    {
        return false;
    }

    public function isOptional(): bool
    {
        return false;
    }

    public function isVariadic(): bool
    {
        return false;
    }

    public function isPassedByReference(): bool
    {
        return false;
    }

    public function allowsNull(): bool
    {
        return true;
    }

    public function isDefaultValueAvailable(): bool
    {
        return false;
    }

    public function getPosition(): int
    {
        return 0;
    }
}

class ReflectionProperty
{
    public const IS_STATIC = 16;
    public const IS_READONLY = 128;
    public const IS_PUBLIC = 1;
    public const IS_PROTECTED = 2;
    public const IS_PRIVATE = 4;

    public string $name = '';
    public string $class = '';

    /** @param object|string $class */
    public function __construct($class, string $property)
    {
        throw new ReflectionException('Properties cannot be reflected in a compiled program');
    }

    public function getName(): string
    {
        return $this->name;
    }

    public function getType(): ?ReflectionType
    {
        return null;
    }

    public function isStatic(): bool
    {
        return false;
    }

    public function isPublic(): bool
    {
        return true;
    }

    public function isProtected(): bool
    {
        return false;
    }

    public function isPrivate(): bool
    {
        return false;
    }

    public function getDeclaringClass(): ReflectionClass
    {
        return new ReflectionClass($this->class);
    }
}

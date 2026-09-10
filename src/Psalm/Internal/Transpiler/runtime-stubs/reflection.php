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
        $object = __rt_new_uninit($this->name);
        if ($object === null) {
            throw new ReflectionException('Class "' . $this->name . '" does not exist');
        }
        return $object;
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

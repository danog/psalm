<?php

declare(strict_types=1);

/**
 * Minimal ReflectionClass used by transpiled code (php-parser's JsonDecoder etc.).
 */
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
}

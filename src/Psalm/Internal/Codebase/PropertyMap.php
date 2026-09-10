<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use function strtolower;

/**
 * @internal
 * @psalm-external-mutation-free
 */
final class PropertyMap
{
    /**
     * @var array<lowercase-string, array<string, string>>|null
     */
    private static ?array $property_map = null;

    /**
     * Gets the method/function call map
     *
     * @return array<lowercase-string, array<string, string>>
     * @psalm-external-mutation-free
     */
    public static function getPropertyMap(): array
    {
        if (self::$property_map !== null) {
            return self::$property_map;
        }

        /** @var array<lowercase-string, array<string, string>> */
        $property_map = Dictionaries::propertyMap();

        self::$property_map = $property_map;

        return self::$property_map;
    }

    /**
     * @psalm-external-mutation-free
     */
    public static function inPropertyMap(string $class_name): bool
    {
        return isset(self::getPropertyMap()[strtolower($class_name)]);
    }
}

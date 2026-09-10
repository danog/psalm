<?php

declare(strict_types=1);

namespace Psalm\Internal\Codebase;

use Psalm\Internal\Dictionaries\CallMap70;
use Psalm\Internal\Dictionaries\CallMap71;
use Psalm\Internal\Dictionaries\CallMap72;
use Psalm\Internal\Dictionaries\CallMap73;
use Psalm\Internal\Dictionaries\CallMap74;
use Psalm\Internal\Dictionaries\CallMap80;
use Psalm\Internal\Dictionaries\CallMap81;
use Psalm\Internal\Dictionaries\CallMap82;
use Psalm\Internal\Dictionaries\CallMap83;
use Psalm\Internal\Dictionaries\CallMap84;
use Psalm\Internal\Dictionaries\CallMap85;
use Psalm\Internal\Dictionaries\ImpureFunctionsList;
use Psalm\Internal\Dictionaries\InternalTaintSinkMap;
use Psalm\Internal\Dictionaries\PropertyMap;
use Psalm\Type\TaintKind;
use UnexpectedValueException;

use function base64_decode;
use function gzdecode;
use function json_decode;

use const JSON_THROW_ON_ERROR;

/**
 * Loads the dictionaries embedded in the generated Psalm\Internal\Dictionaries classes.
 *
 * The dictionaries directory remains the source of truth; regenerate the embedded
 * copies with bin/generate-dictionaries.php after changing it.
 *
 * @internal
 */
final class Dictionaries
{
    /**
     * @param int<70, 85> $version
     * @return non-empty-array<lowercase-string, array<int|string, string>>
     * @psalm-pure
     */
    public static function callMap(int $version): array
    {
        $data = match ($version) {
            70 => CallMap70::DATA,
            71 => CallMap71::DATA,
            72 => CallMap72::DATA,
            73 => CallMap73::DATA,
            74 => CallMap74::DATA,
            80 => CallMap80::DATA,
            81 => CallMap81::DATA,
            82 => CallMap82::DATA,
            83 => CallMap83::DATA,
            84 => CallMap84::DATA,
            85 => CallMap85::DATA,
        };

        /** @var non-empty-array<lowercase-string, array<int|string, string>> */
        return self::decode($data);
    }

    /**
     * @return array<string, array<string, string>>
     * @psalm-pure
     */
    public static function propertyMap(): array
    {
        /** @var array<string, array<string, string>> */
        return self::decode(PropertyMap::DATA);
    }

    /**
     * @return array<string, true>
     * @psalm-pure
     */
    public static function impureFunctions(): array
    {
        /** @var array<string, true> */
        return self::decode(ImpureFunctionsList::DATA);
    }

    /**
     * @return non-empty-array<string, non-empty-list<int-mask-of<TaintKind::*>>>
     * @psalm-pure
     */
    public static function taintSinkMap(): array
    {
        /** @var non-empty-array<string, non-empty-list<int-mask-of<TaintKind::*>>> */
        return self::decode(InternalTaintSinkMap::DATA);
    }

    /**
     * @psalm-pure
     */
    private static function decode(string $data): array
    {
        $json = gzdecode(base64_decode($data, false));
        if ($json === false) {
            throw new UnexpectedValueException('Corrupt embedded dictionary');
        }

        /** @var array */
        return json_decode($json, true, 512, JSON_THROW_ON_ERROR);
    }
}

<?php

declare(strict_types=1);

namespace Psalm\Tests\Internal;

use Psalm\Tests\TestCase;

use function strnatcasecmp;
use function uksort;

/**
 * @psalm-type TCallMap=array<string, array<int|string, string>>
 * @psalm-type TCallMaps=array<int, array<string, array<int|string, string>>>
 */
final class CallMapTest extends TestCase
{
    protected const DICTIONARY_PATH = 'dictionaries';

    public function testDictionaryPathMustBeAReadableDirectory(): void
    {
        self::assertDirectoryExists(self::DICTIONARY_PATH, self::DICTIONARY_PATH . " is not a valid directory");
        self::assertDirectoryIsReadable(self::DICTIONARY_PATH, self::DICTIONARY_PATH . " is not a readable directory");
    }

    /**
     * @depends testDictionaryPathMustBeAReadableDirectory
     * @return array<int, TCallMap>
     */
    public function testLoadCallMaps(): array
    {
        // the call map versions are compiled into the program: listed explicitly (closed world)
        /** @var array<string, TCallMap> */
        $deltaFiles = [
            'CallMap_70.php' => require dirname(__DIR__, 2) . '/dictionaries/CallMap_70.php',
            'CallMap_71.php' => require dirname(__DIR__, 2) . '/dictionaries/CallMap_71.php',
            'CallMap_72.php' => require dirname(__DIR__, 2) . '/dictionaries/CallMap_72.php',
            'CallMap_73.php' => require dirname(__DIR__, 2) . '/dictionaries/CallMap_73.php',
            'CallMap_74.php' => require dirname(__DIR__, 2) . '/dictionaries/CallMap_74.php',
            'CallMap_80.php' => require dirname(__DIR__, 2) . '/dictionaries/CallMap_80.php',
            'CallMap_81.php' => require dirname(__DIR__, 2) . '/dictionaries/CallMap_81.php',
            'CallMap_82.php' => require dirname(__DIR__, 2) . '/dictionaries/CallMap_82.php',
            'CallMap_83.php' => require dirname(__DIR__, 2) . '/dictionaries/CallMap_83.php',
            'CallMap_84.php' => require dirname(__DIR__, 2) . '/dictionaries/CallMap_84.php',
            'CallMap_85.php' => require dirname(__DIR__, 2) . '/dictionaries/CallMap_85.php',
        ];

        uksort($deltaFiles, strnatcasecmp(...));

        return $deltaFiles;
    }

    /**
     * @depends testLoadCallMaps
     * @param TCallMaps $callMaps
     */
    public function testSignatureKeysAreZeroOrStringAndValuesAreTypes(array $callMaps): void
    {
        foreach ($callMaps as $callMap) {
            foreach ($callMap as $function => $signature) {
                self::assertArrayKeysAreZeroOrString($signature, "Function " . $function . " in main CallMap has invalid keys");
                self::assertArrayValuesAreStrings($signature, "Function " . $function . " in main CallMap has non-string values");
            }
        }
    }

    /**
     * @depends testLoadCallMaps
     * @param TCallMaps $callMaps
     */
    public function testTypesAreParsable(array $callMaps): void
    {
        foreach ($callMaps as $callMap) {
            foreach ($callMap as $function => $signature) {
                foreach ($signature as $type) {
                    self::assertStringIsParsableType($type, "Function " . $function . " in main CallMap contains invalid type declaration " . $type);
                }
            }
        }
    }
}

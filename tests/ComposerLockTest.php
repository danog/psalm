<?php

declare(strict_types=1);

namespace Psalm\Tests;

use Psalm\Internal\PluginManager\ComposerLock;
use RuntimeException;

use function json_encode;

use const JSON_THROW_ON_ERROR;

/** @group PluginManager */
/**
 * @psalm-type Package = array{name: string, type?: string, extra?: array{psalm: array{pluginClass: string}}}
 */
final class ComposerLockTest extends TestCase
{
    /**
     * @test
     */
    public function packageIsPsalmPlugin(): void
    {
        $lock = new ComposerLock([$this->jsonFile([])]);

        $this->assertTrue($lock->isPlugin([
            'name' => 'vendor/package',
            'type' => 'psalm-plugin',
            'extra' => [
                'psalm' => [
                    'pluginClass' => 'Some\Class',
                ],
            ],
        ]), 'Non-plugin should not be considered a plugin');

        $this->assertTrue($lock->isPlugin([
            'name' => 'vendor/package',
            'type' => 'library',
            'extra' => [
                'psalm' => [
                    'pluginClass' => 'Some\Class',
                ],
            ],
        ]), 'Non-plugin should not be considered a plugin');

        $this->assertTrue($lock->isPlugin([
            'name' => 'vendor/package',
            'extra' => [
                'psalm' => [
                    'pluginClass' => 'Some\Class',
                ],
            ],
        ]), 'Non-plugin should not be considered a plugin');

        // counterexamples

        $this->assertFalse($lock->isPlugin([]), 'Non-package should not be considered a plugin');

        $this->assertFalse($lock->isPlugin([
            'name' => 'vendor/package',
            'type' => 'library',
        ]), 'Non-plugin should not be considered a plugin');

        $this->assertFalse($lock->isPlugin([
            'name' => 'vendor/package',
            'type' => 'psalm-plugin',
        ]), 'Invalid plugin should not be considered a plugin');
    }

    /**
     * @test
     */
    public function seesNonDevPlugins(): void
    {
        $lock = new ComposerLock([$this->jsonFile([
            'packages' => [
                $this->pluginEntry('vendor/package', 'Vendor\Package\PluginClass'),
            ],
            'packages-dev' => [],
        ])]);

        $plugins = $lock->getPlugins();
        $this->assertArrayHasKey('vendor/package', $plugins);
        $this->assertSame('Vendor\Package\PluginClass', $plugins['vendor/package']);
    }

    /**
     * @test
     */
    public function seesDevPlugins(): void
    {
        $lock = new ComposerLock([$this->jsonFile([
            'packages' => [],
            'packages-dev' => [
                $this->pluginEntry('vendor/package', 'Vendor\Package\PluginClass'),
            ],
        ])]);

        $plugins = $lock->getPlugins();
        $this->assertArrayHasKey('vendor/package', $plugins);
        $this->assertSame('Vendor\Package\PluginClass', $plugins['vendor/package']);
    }

    /**
     * @test
     */
    public function skipsNonPlugins(): void
    {
        $nonPlugin = [
            'name' => 'vendor/package',
            'type' => 'library',
        ];

        $lock = new ComposerLock([$this->jsonFile([
            'packages' => [$nonPlugin],
            'packages-dev' => [$nonPlugin],
        ])]);
        $this->assertEmpty($lock->getPlugins());
    }

    /**
     * @test
     */
    public function failsOnInvalidJson(): void
    {
        $lock = new ComposerLock(['data:application/json,[']);

        $this->expectException(RuntimeException::class);
        $lock->getPlugins();
    }

    /**
     * @test
     */
    public function failsOnNonObjectJson(): void
    {
        $lock = new ComposerLock(['data:application/json,null']);

        $this->expectException(RuntimeException::class);
        $lock->getPlugins();
    }

    /**
     * @test
     */
    public function failsOnMissingPackagesEntry(): void
    {
        $noPackagesFile = $this->jsonFile([
            'packages-dev' => [],
        ]);
        $lock = new ComposerLock([$noPackagesFile]);
        $this->expectException(RuntimeException::class);
        $lock->getPlugins();
    }

    /**
     * @test
     */
    public function failsOnMissingPackagesDevEntry(): void
    {
        $noPackagesDevFile = $this->jsonFile([
            'packages' => [],
        ]);
        $lock = new ComposerLock([$noPackagesDevFile]);
        $this->expectException(RuntimeException::class);
        $lock->getPlugins();
    }

    /** @test */
    public function mergesMultipleComposerLockFiles(): void
    {
        $lock = new ComposerLock([
            $this->jsonFile([
                'packages' => [
                    $this->pluginEntry('vendor/packageA', 'Vendor\PackageA\PluginClass'),
                ],
                'packages-dev' => [
                    $this->pluginEntry('vendor/packageB', 'Vendor\PackageB\PluginClass'),
                ],
            ]),
            $this->jsonFile([
                'packages' => [
                    $this->pluginEntry('vendor/packageC', 'Vendor\PackageC\PluginClass'),
                ],
                'packages-dev' => [
                    $this->pluginEntry('vendor/packageD', 'Vendor\PackageD\PluginClass'),
                ],
            ]),
        ]);

        $this->assertSame(
            [
                'vendor/packageA' => 'Vendor\PackageA\PluginClass',
                'vendor/packageB' => 'Vendor\PackageB\PluginClass',
                'vendor/packageC' => 'Vendor\PackageC\PluginClass',
                'vendor/packageD' => 'Vendor\PackageD\PluginClass',
            ],
            $lock->getPlugins(),
        );
    }

    /**
     * @psalm-pure
     * @return Package
     */
    private function pluginEntry(string $package_name, string $package_class): array
    {
        return [
            'name' => $package_name,
            'type' => 'psalm-plugin',
            'extra' => [
                'psalm' => [
                    'pluginClass' => $package_class,
                ],
            ],
        ];
    }

    /**
     * @psalm-pure
     * @param array{packages?: list<Package>, packages-dev?: list<Package>} $data
     */
    private function jsonFile(array $data): string
    {
        return 'data:application/json,' . json_encode($data, JSON_THROW_ON_ERROR);
    }
}

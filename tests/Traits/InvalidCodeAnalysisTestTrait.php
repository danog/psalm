<?php
namespace Psalm\Tests\Traits;

use function is_int;
use const PHP_VERSION;
use function preg_quote;
use function preg_replace;
use Psalm\Config;
use Psalm\Context;
use function strpos;
use function version_compare;

trait InvalidCodeAnalysisTestTrait
{
    /**
     * @return iterable<string,array{string,error_message:string,2?:string[],3?:bool,4?:string}>
     */
    abstract public function providerInvalidCodeParse(): iterable;

    /**
     * @dataProvider providerInvalidCodeParse
     * @small
     *
     * @param string $code
     * @param string $error_message
     * @param array<int|string, string> $error_levels
     * @param bool $strict_mode
     *
     * @return void
     */
    public function testInvalidCode(
        $code,
        $error_message,
        $error_levels = [],
        $strict_mode = false,
        string $php_version = '7.3'
    ) {
        $test_name = $this->getTestName();
        if (strpos($test_name, 'PHP71-') !== false) {
            if (version_compare(PHP_VERSION, '7.1.0', '<')) {
                $this->markTestSkipped('Test case requires PHP 7.1.');

                return;
            }
        } elseif (strpos($test_name, 'SKIPPED-') !== false) {
            $this->markTestSkipped('Skipped due to a bug.');
        }

        if (\strtoupper(\substr(\PHP_OS, 0, 3)) === 'WIN') {
            $code = \str_replace("\n", "\r\n", $code);
        }

        if ($strict_mode) {
            Config::getInstance()->strict_binary_operands = true;
        }

        foreach ($error_levels as $error_level_key => $error_level) {
            if (is_int($error_level_key)) {
                $issue_name = $error_level;
                $error_level = Config::REPORT_SUPPRESS;
            } else {
                $issue_name = $error_level_key;
            }

            Config::getInstance()->setCustomErrorLevel($issue_name, $error_level);
        }

        $this->project_analyzer->setPhpVersion($php_version);

        $file_path = self::$src_dir_path . 'somefile.php';

        // $error_message = preg_replace('/ src[\/\\\\]somefile\.php/', ' src/somefile.php', $error_message);

        $this->expectException(\Psalm\Exception\CodeException::class);

        if (\method_exists($this, 'expectExceptionMessageMatches')) {
            $this->expectExceptionMessageMatches('/\b' . preg_quote($error_message, '/') . '\b/');
        } else {
            $this->expectExceptionMessageRegExp('/\b' . preg_quote($error_message, '/') . '\b/');
        }

        $this->addFile($file_path, $code);
        $this->analyzeFile($file_path, new Context());
    }
}

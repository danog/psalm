<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use Psalm\Plugin\EventHandler\AfterClassLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use SimpleXMLElement;

/**
 * Adds an attribute to php-parser's NodeAttributes, next to an existing one (env NODE_ATTRIBUTE = JSON
 * [new name, PHP type, psalm type, existing name to place it after]): the property, its fromArray and toArray
 * entries, and its key in the AttributeArray psalm-type.
 */
final class NodeAttributePlugin implements PluginEntryPointInterface, AfterClassLikeAnalysisInterface
{
    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function afterStatementAnalysis(AfterClassLikeAnalysisEvent $event): ?bool
    {
        if (strcasecmp(\Psalm\Internal\Interner::lookup($event->getClasslikeStorage()->id), 'PhpParser\NodeAttributes') !== 0) {
            return null;
        }
        [$name, $php_type, $psalm_type, $after] = json_decode((string) getenv('NODE_ATTRIBUTE'), true);
        $file = $event->getStatementsSource()->getFilePath();
        $src = (string) file_get_contents($file);
        if (str_contains($src, '$' . $name . ' ')) {
            return null;
        }
        $edits = [];
        $at = function (string $needle, int $from = 0) use ($src): int {
            $p = strpos($src, $needle, $from);
            if ($p === false) {
                throw new \RuntimeException('not found: ' . $needle);
            }
            return $p;
        };
        // the psalm-type key
        $p = $at($after . '?: ');
        $end = strpos($src, ', ', $p) + 2;
        $edits[] = [$end, $end, $name . '?: ' . $psalm_type . ', '];
        // the property
        $p = $at('$' . $after . ' = null;');
        $eol = strpos($src, "\n", $p) + 1;
        $edits[] = [$eol, $eol, '    public ' . $php_type . ' $' . $name . " = null;\n"];
        // fromArray
        $p = $at("if (isset(\$attributes['" . $after . "'])) {");
        $close = strpos($src, "        }\n", $p) + strlen("        }\n");
        $edits[] = [$close, $close, "        if (isset(\$attributes['" . $name . "'])) {\n            \$a->" . $name . " = \$attributes['" . $name . "'];\n        }\n"];
        // toArray
        $p = $at('if ($this->' . $after . ' !== null) {');
        $close = strpos($src, "        }\n", $p) + strlen("        }\n");
        $edits[] = [$close, $close, '        if ($this->' . $name . " !== null) {\n            \$out['" . $name . "'] = \$this->" . $name . ";\n        }\n"];
        file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/node-attr.jsonl', json_encode(
            ['kind' => 'edit', 'file' => $file, 'site' => $file . ':attr:' . $name, 'edits' => $edits], JSON_UNESCAPED_SLASHES) . "\n",
            FILE_APPEND | LOCK_EX);
        return null;
    }
}

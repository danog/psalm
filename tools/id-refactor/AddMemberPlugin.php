<?php

declare(strict_types=1);

namespace Psalm\Tools\IdRefactor;

use Psalm\Plugin\EventHandler\AfterClassLikeAnalysisInterface;
use Psalm\Plugin\EventHandler\Event\AfterClassLikeAnalysisEvent;
use Psalm\Plugin\PluginEntryPointInterface;
use Psalm\Plugin\RegistrationInterface;
use SimpleXMLElement;

/**
 * Adds class members: env ADD_MEMBERS = JSON list of [class, member name, PHP source of the member]. A member is
 * added after the class's last method unless the class already declares a method of that name.
 */
final class AddMemberPlugin implements PluginEntryPointInterface, AfterClassLikeAnalysisInterface
{
    public function __invoke(RegistrationInterface $registration, ?SimpleXMLElement $config = null): void
    {
        $registration->registerHooksFromClass(self::class);
    }

    public static function afterStatementAnalysis(AfterClassLikeAnalysisEvent $event): ?bool
    {
        $stmt = $event->getStmt();
        $name = \Psalm\Internal\Interner::lookup($event->getClasslikeStorage()->id);
        $file = $event->getStatementsSource()->getFilePath();
        foreach (json_decode((string) getenv('ADD_MEMBERS'), true) ?: [] as [$class, $member, $code]) {
            if (strcasecmp($class, $name) !== 0 || $stmt->getMethod($member) !== null) {
                continue;
            }
            $methods = $stmt->getMethods();
            $at = $methods !== [] ? end($methods)->getEndFilePos() + 1 : $stmt->getEndFilePos();
            file_put_contents(getenv('ID_REFACTOR_OUT') ?: sys_get_temp_dir() . '/add-member.jsonl', json_encode([
                'kind' => 'edit', 'file' => $file, 'site' => "$class::$member",
                'edits' => [[$at, $at, "\n\n" . rtrim($code)]],
            ], JSON_UNESCAPED_SLASHES) . "\n", FILE_APPEND | LOCK_EX);
        }
        return null;
    }
}

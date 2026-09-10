<?php

declare(strict_types=1);

namespace Symfony\Component\Console\Command;

class Command implements \Symfony\Component\Console\Command\SignalableCommandInterface
{
    public const SUCCESS = 0;
    public const FAILURE = 1;
    public const INVALID = 2;
    private ?\Symfony\Component\Console\Application $application = NULL;
    private ?string $name = NULL;
    private ?string $processTitle = NULL;
    private array $aliases = array (
);
    private \Symfony\Component\Console\Input\InputDefinition $definition;
    private bool $hidden = false;
    private string $help = '';
    private string $description = '';
    private ?\Symfony\Component\Console\Input\InputDefinition $fullDefinition = NULL;
    private bool $ignoreValidationErrors = false;
    private ?\Symfony\Component\Console\Command\InvokableCommand $code = NULL;
    private array $synopsis = array (
);
    private array $usages = array (
);
    private ?\Symfony\Component\Console\Helper\HelperSet $helperSet = NULL;
    public static function getDefaultName(): ?string
    {
        return null;
    }
    public static function getDefaultDescription(): ?string
    {
        return null;
    }
    public function __construct(?string $name = NULL, ?callable $code = NULL)
    {
        throw new \RuntimeException('vendor stub');
    }
    public function ignoreValidationErrors(): void
    {
        throw new \RuntimeException('vendor stub');
    }
    public function setApplication(?\Symfony\Component\Console\Application $application): void
    {
        throw new \RuntimeException('vendor stub');
    }
    public function setHelperSet(\Symfony\Component\Console\Helper\HelperSet $helperSet): void
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getHelperSet(): ?\Symfony\Component\Console\Helper\HelperSet
    {
        return null;
    }
    public function getApplication(): ?\Symfony\Component\Console\Application
    {
        return null;
    }
    public function isEnabled(): bool
    {
        return false;
    }
    protected function configure()
    {
        throw new \RuntimeException('vendor stub');
    }
    protected function execute(\Symfony\Component\Console\Input\InputInterface $input, \Symfony\Component\Console\Output\OutputInterface $output): int
    {
        return 0;
    }
    protected function interact(\Symfony\Component\Console\Input\InputInterface $input, \Symfony\Component\Console\Output\OutputInterface $output)
    {
        throw new \RuntimeException('vendor stub');
    }
    protected function initialize(\Symfony\Component\Console\Input\InputInterface $input, \Symfony\Component\Console\Output\OutputInterface $output)
    {
        throw new \RuntimeException('vendor stub');
    }
    public function run(\Symfony\Component\Console\Input\InputInterface $input, \Symfony\Component\Console\Output\OutputInterface $output): int
    {
        return 0;
    }
    public function complete(\Symfony\Component\Console\Completion\CompletionInput $input, \Symfony\Component\Console\Completion\CompletionSuggestions $suggestions): void
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getCode(): ?callable
    {
        return null;
    }
    public function setCode(callable $code): static
    {
        throw new \RuntimeException('vendor stub');
    }
    public function mergeApplicationDefinition(bool $mergeArgs = true): void
    {
        throw new \RuntimeException('vendor stub');
    }
    public function setDefinition(\Symfony\Component\Console\Input\InputDefinition|array $definition): static
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getDefinition(): \Symfony\Component\Console\Input\InputDefinition
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getNativeDefinition(): \Symfony\Component\Console\Input\InputDefinition
    {
        throw new \RuntimeException('vendor stub');
    }
    public function addArgument(string $name, ?int $mode = NULL, string $description = '', mixed $default = NULL, \Closure|array $suggestedValues = array (
)): static
    {
        throw new \RuntimeException('vendor stub');
    }
    public function addOption(string $name, array|string|null $shortcut = NULL, ?int $mode = NULL, string $description = '', mixed $default = NULL, \Closure|array $suggestedValues = array (
)): static
    {
        throw new \RuntimeException('vendor stub');
    }
    public function setName(string $name): static
    {
        throw new \RuntimeException('vendor stub');
    }
    public function setProcessTitle(string $title): static
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getName(): ?string
    {
        return null;
    }
    public function setHidden(bool $hidden = true): static
    {
        throw new \RuntimeException('vendor stub');
    }
    public function isHidden(): bool
    {
        return false;
    }
    public function setDescription(string $description): static
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getDescription(): string
    {
        return '';
    }
    public function setHelp(string $help): static
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getHelp(): string
    {
        return '';
    }
    public function getProcessedHelp(): string
    {
        return '';
    }
    public function setAliases(iterable $aliases): static
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getAliases(): array
    {
        return [];
    }
    public function getSynopsis(bool $short = false): string
    {
        return '';
    }
    public function addUsage(string $usage): static
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getUsages(): array
    {
        return [];
    }
    public function getHelper(string $name): \Symfony\Component\Console\Helper\HelperInterface
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubscribedSignals(): array
    {
        return [];
    }
    public function handleSignal(int $signal, int|false $previousExitCode = 0): int|false
    {
        throw new \RuntimeException('vendor stub');
    }
    private function validateName(string $name): void
    {
        throw new \RuntimeException('vendor stub');
    }
}

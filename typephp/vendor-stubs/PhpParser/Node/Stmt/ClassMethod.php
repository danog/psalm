<?php

declare(strict_types=1);

namespace PhpParser\Node\Stmt;

class ClassMethod extends \PhpParser\Node\Stmt implements \PhpParser\Node\FunctionLike
{
    public int $flags;
    public bool $byRef;
    public \PhpParser\Node\Identifier $name;
    public array $params;
    public ?\PhpParser\Node $returnType = null;
    public ?array $stmts = null;
    public array $attrGroups;
    private static array $magicNames = array (
  '__construct' => true,
  '__destruct' => true,
  '__call' => true,
  '__callstatic' => true,
  '__get' => true,
  '__set' => true,
  '__isset' => true,
  '__unset' => true,
  '__sleep' => true,
  '__wakeup' => true,
  '__tostring' => true,
  '__set_state' => true,
  '__clone' => true,
  '__invoke' => true,
  '__debuginfo' => true,
  '__serialize' => true,
  '__unserialize' => true,
);
    public function __construct($name, array $subNodes = array (
), array $attributes = array (
))
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getSubNodeNames(): array
    {
        return [];
    }
    public function returnsByRef(): bool
    {
        return false;
    }
    public function getParams(): array
    {
        return [];
    }
    public function getReturnType()
    {
        throw new \RuntimeException('vendor stub');
    }
    public function getStmts(): ?array
    {
        return null;
    }
    public function getAttrGroups(): array
    {
        return [];
    }
    public function isPublic(): bool
    {
        return false;
    }
    public function isProtected(): bool
    {
        return false;
    }
    public function isPrivate(): bool
    {
        return false;
    }
    public function isAbstract(): bool
    {
        return false;
    }
    public function isFinal(): bool
    {
        return false;
    }
    public function isStatic(): bool
    {
        return false;
    }
    public function isMagic(): bool
    {
        return false;
    }
    public function getType(): string
    {
        return '';
    }
}

<?php

declare(strict_types=1);

namespace AdvancedJsonRpc;

class Dispatcher
{
    private $target = NULL;
    private $delimiter = NULL;
    private $methods = NULL;
    private $docBlockFactory = NULL;
    private $contextFactory = NULL;
    private \JsonMapper $mapper;
    public function __construct($target, $delimiter = '->')
    {
        throw new \RuntimeException('vendor stub');
    }
    public function dispatch($msg)
    {
        throw new \RuntimeException('vendor stub');
    }
}

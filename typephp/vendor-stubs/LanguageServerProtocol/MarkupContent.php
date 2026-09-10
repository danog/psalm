<?php

declare(strict_types=1);

namespace LanguageServerProtocol;

class MarkupContent
{
    public $kind = NULL;
    public $value = NULL;
    public function __construct(?string $kind = NULL, ?string $value = NULL)
    {
        throw new \RuntimeException('vendor stub');
    }
}

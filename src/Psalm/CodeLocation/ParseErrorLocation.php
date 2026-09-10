<?php

declare(strict_types=1);

namespace Psalm\CodeLocation;

use PhpParser;
use Psalm\CodeLocation;

use function substr;
use function substr_count;

/**
 * @psalm-immutable
 */
final class ParseErrorLocation extends CodeLocation
{
    /**
     * @psalm-mutation-free
     */
    public function __construct(
        PhpParser\Error $error,
        string $file_contents,
        string $file_path,
        string $file_name,
    ) {
        $this->file_start = $error->getAttributes()->startFilePos ?? 0;
        $this->file_end = $error->getAttributes()->endFilePos ?? 0;
        $this->raw_file_start = $this->file_start;
        $this->raw_file_end = $this->file_end;
        $this->file_path = $file_path;
        $this->file_name = $file_name;
        $this->single_line = false;

        $this->preview_start = $this->file_start;
        $this->raw_line_number = substr_count(
            substr($file_contents, 0, $this->file_start),
            "\n",
        ) + 1;
    }
}

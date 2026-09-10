<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Expr;

/**
 * A file that can be the target of `include`/`require`: either a data file (`<?php return [...];`) whose
 * value is compiled into a static table, or a unit of declarations whose inclusion yields `1`.
 *
 * @internal
 */
final class FileModel
{
    public function __construct(
        /** path relative to the source root */
        public readonly string $rel_path,
        public readonly string $abs_path,
        public readonly int $crate,
        /** the returned expression of a data file; null for declaration-only files and mechanical data files */
        public readonly ?Expr $data,
        /** a dictionary whose value is obtained by running the file (never parsed) */
        public readonly bool $mechanical = false,
    ) {
    }

    public function isData(): bool
    {
        return $this->data !== null || $this->mechanical;
    }

    /** Rust name of the file function (data files only). */
    public function rustName(): string
    {
        return 'f_' . Names::ident(pathinfo($this->rel_path, PATHINFO_FILENAME)) . '_' . substr(md5($this->rel_path), 0, 8);
    }

    public function path(): string
    {
        return 'crate::files::' . $this->rustName();
    }
}

# Hand-written vendor replacements

Files in this directory replace the vendor file at the same relative path in
the closed-world native build (see `typephp/gen-vendor-build.php`). They exist
only where the vendor code relies on dynamic features that TypePHP does not
support (`Closure::bind()`, by-reference captures of dynamic locals, ...).

Keep each replacement as close as possible to the original and re-check them
after updating the corresponding package.

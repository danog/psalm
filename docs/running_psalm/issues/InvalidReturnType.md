# InvalidReturnType

Emitted when a function's declared return type cannot be right for structural reasons: not every code path
ends in a return statement, no return statement exists at all, a `never` function returns, or a generator's
aggregated yield/return type is wrong. A `return` whose value does not fit the declared type is reported at
that statement instead (`InvalidReturnStatement`, `NullableReturnStatement`, `FalsableReturnStatement`,
`LessSpecificReturnStatement`).

```php
<?php

function foo(int $i) : int {
    if ($i > 0) {
        return $i;
    }
}
```

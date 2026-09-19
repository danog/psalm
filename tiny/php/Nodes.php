<?php

/**
 * A replica of php-parser's ShellExec/InterpolatedString pair: both declare the same property type,
 * but the full transpile gives ShellExec's only one of the two members.
 */

namespace Tiny\Node {
    abstract class NodeAbstract
    {
    }

    abstract class Expr extends NodeAbstract
    {
    }

    abstract class Scalar extends Expr
    {
    }

    class InterpolatedStringPart extends NodeAbstract
    {
        public function __construct(public string $value)
        {
        }
    }

    \class_alias(InterpolatedStringPart::class, Scalar\EncapsedStringPart::class);
}

namespace Tiny\Node\Expr {

    use Tiny\Node\Expr;
    use Tiny\Node\InterpolatedStringPart;

    class ShellExec extends Expr
    {
        /** @var list<Expr|InterpolatedStringPart> Interpolated string array */
        public array $parts;

        /** @param list<Expr|InterpolatedStringPart> $parts Interpolated string array */
        public function __construct(array $parts)
        {
            $this->parts = $parts;
        }
    }

    final class Variable extends Expr
    {
        public function __construct(public string $name)
        {
        }
    }
}

namespace Tiny\Virtual {

    final class VirtualShellExec extends \Tiny\Node\Expr\ShellExec
    {
    }

    final class VirtualInterpolatedString extends \Tiny\Node\Scalar\InterpolatedString
    {
    }
}

namespace Tiny\Node\Scalar {

    use Tiny\Node\Expr;
    use Tiny\Node\InterpolatedStringPart;
    use Tiny\Node\Scalar;

    class InterpolatedString extends Scalar
    {
        /** @var list<Expr|InterpolatedStringPart> list of string parts */
        public array $parts;

        /** @param list<Expr|InterpolatedStringPart> $parts list of string parts */
        public function __construct(array $parts)
        {
            $this->parts = $parts;
        }
    }
}

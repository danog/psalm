<?php

declare(strict_types=1);

/**
 * SimpleXML / DOM runtime stubs: a shared mutable element tree (XmlNode) parsed natively by the
 * Rust runtime (`__rt_xml_parse`), exposed through the subset of the SimpleXMLElement and
 * DOMDocument APIs that Psalm uses.
 */

/** @internal */
final class XmlNode
{
    /** @var array<string, string> */
    public array $attrs = [];

    /** @var list<XmlNode> */
    public array $children = [];

    public ?XmlNode $parent = null;

    public function __construct(
        public string $name,
        public string $text = '',
        public bool $is_text = false,
    ) {
    }

    /** @param mixed $tree */
    public static function fromTree($tree, ?XmlNode $parent = null): XmlNode
    {
        /** @var array{name: string, attrs: array<string, string>, children: list<mixed>, text: string} $tree */
        $node = new XmlNode($tree['name'], $tree['text'], $tree['name'] === '#text');
        $node->parent = $parent;
        $node->attrs = $tree['attrs'];
        foreach ($tree['children'] as $child) {
            $node->children[] = self::fromTree($child, $node);
        }
        return $node;
    }

    public static function parse(string $xml): ?XmlNode
    {
        $tree = __rt_xml_parse($xml);
        if ($tree === null) {
            return null;
        }
        return self::fromTree($tree);
    }

    /** @return list<XmlNode> */
    public function elementChildren(?string $name = null): array
    {
        $out = [];
        foreach ($this->children as $child) {
            if (!$child->is_text && ($name === null || $child->name === $name)) {
                $out[] = $child;
            }
        }
        return $out;
    }

    /** Text content: concatenated text of all descendants. */
    public function textContent(): string
    {
        if ($this->is_text) {
            return $this->text;
        }
        $out = '';
        foreach ($this->children as $child) {
            $out .= $child->textContent();
        }
        return $out;
    }

    public function appendChild(XmlNode $child): void
    {
        if ($child->parent !== null) {
            $child->parent->removeChild($child);
        }
        $child->parent = $this;
        $this->children[] = $child;
    }

    public function removeChild(XmlNode $child): bool
    {
        foreach ($this->children as $i => $c) {
            if ($c === $child) {
                array_splice($this->children, $i, 1);
                $child->parent = null;
                return true;
            }
        }
        return false;
    }

    /** @return list<XmlNode> */
    public function descendantsNamed(string $name): array
    {
        $out = [];
        foreach ($this->children as $child) {
            if ($child->is_text) {
                continue;
            }
            if ($name === '*' || $child->name === $name) {
                $out[] = $child;
            }
            foreach ($child->descendantsNamed($name) as $d) {
                $out[] = $d;
            }
        }
        return $out;
    }

    public function serialize(bool $pretty, int $depth = 0): string
    {
        if ($this->is_text) {
            return __rt_xml_escape($this->text, false);
        }
        $indent = $pretty ? str_repeat('  ', $depth) : '';
        $out = $indent . '<' . $this->name;
        foreach ($this->attrs as $k => $v) {
            $out .= ' ' . $k . '="' . __rt_xml_escape($v, true) . '"';
        }
        if ($this->children === []) {
            return $out . '/>' . ($pretty ? "\n" : '');
        }
        $out .= '>';
        $only_text = true;
        foreach ($this->children as $child) {
            if (!$child->is_text) {
                $only_text = false;
            }
        }
        if ($only_text || !$pretty) {
            foreach ($this->children as $child) {
                $out .= $child->serialize(false, $depth + 1);
            }
            return $out . '</' . $this->name . '>' . ($pretty ? "\n" : '');
        }
        $out .= "\n";
        foreach ($this->children as $child) {
            if ($child->is_text) {
                if (trim($child->text) !== '') {
                    $out .= $indent . '  ' . __rt_xml_escape(trim($child->text), false) . "\n";
                }
                continue;
            }
            $out .= $child->serialize(true, $depth + 1);
        }
        return $out . $indent . '</' . $this->name . ">\n";
    }
}

/**
 * @implements ArrayAccess<array-key, SimpleXMLElement|string|null>
 * @implements Iterator<string, SimpleXMLElement>
 */
class SimpleXMLElement implements Stringable, Countable, ArrayAccess, Iterator
{
    /** @var list<XmlNode> */
    private array $nodes = [];

    /** @var list<array{string, string}>|null attribute view (`attributes()`): name/value pairs */
    private ?array $attr_pairs = null;

    private ?string $attr_value = null;

    private int $pos = 0;

    public function __construct(string $data = '<root/>', int $options = 0, bool $dataIsURL = false)
    {
        if ($dataIsURL) {
            $contents = file_get_contents($data);
            $data = $contents === false ? '' : $contents;
        }
        $root = XmlNode::parse($data);
        if ($root === null) {
            throw new Exception('String could not be parsed as XML');
        }
        $this->nodes = [$root];
    }

    /**
     * @internal
     * @param list<XmlNode> $nodes
     */
    public static function fromNodes(array $nodes): SimpleXMLElement
    {
        $el = new SimpleXMLElement('<x/>');
        $el->nodes = $nodes;
        return $el;
    }

    /** @internal */
    public static function fromAttribute(string $value): SimpleXMLElement
    {
        $el = new SimpleXMLElement('<x/>');
        $el->nodes = [];
        $el->attr_value = $value;
        return $el;
    }

    /** @internal */
    public function firstNode(): ?XmlNode
    {
        return $this->nodes[0] ?? null;
    }

    public function __get(string $name): SimpleXMLElement
    {
        $first = $this->nodes[0] ?? null;
        return self::fromNodes($first === null ? [] : $first->elementChildren($name));
    }

    public function __isset(string $name): bool
    {
        $first = $this->nodes[0] ?? null;
        return $first !== null && $first->elementChildren($name) !== [];
    }

    public function getName(): string
    {
        $first = $this->nodes[0] ?? null;
        return $first === null ? '' : $first->name;
    }

    public function children(?string $namespaceOrPrefix = null, bool $isPrefix = false): SimpleXMLElement
    {
        $first = $this->nodes[0] ?? null;
        return self::fromNodes($first === null ? [] : $first->elementChildren());
    }

    public function attributes(?string $namespaceOrPrefix = null, bool $isPrefix = false): SimpleXMLElement
    {
        $first = $this->nodes[0] ?? null;
        $el = self::fromNodes([]);
        $pairs = [];
        if ($first !== null) {
            foreach ($first->attrs as $k => $v) {
                $pairs[] = [$k, $v];
            }
        }
        $el->attr_pairs = $pairs;
        return $el;
    }

    public function addChild(string $qualifiedName, ?string $value = null, ?string $namespace = null): ?SimpleXMLElement
    {
        $first = $this->nodes[0] ?? null;
        if ($first === null) {
            return null;
        }
        $child = new XmlNode($qualifiedName);
        if ($value !== null) {
            $child->appendChild(new XmlNode('#text', $value, true));
        }
        $first->appendChild($child);
        return self::fromNodes([$child]);
    }

    public function addAttribute(string $qualifiedName, string $value, ?string $namespace = null): void
    {
        $first = $this->nodes[0] ?? null;
        if ($first !== null) {
            $first->attrs[$qualifiedName] = $value;
        }
    }

    /** @return string|false */
    public function asXML(?string $filename = null)
    {
        $first = $this->nodes[0] ?? null;
        if ($first === null) {
            return false;
        }
        $xml = $first->serialize(false);
        if ($first->parent === null) {
            $xml = "<?xml version=\"1.0\"?>\n" . $xml . "\n";
        }
        if ($filename !== null) {
            return file_put_contents($filename, $xml) !== false;
        }
        return $xml;
    }

    public function __toString(): string
    {
        if ($this->attr_value !== null) {
            return $this->attr_value;
        }
        $first = $this->nodes[0] ?? null;
        if ($first === null) {
            return '';
        }
        // direct text content only (as SimpleXML does)
        $out = '';
        foreach ($first->children as $child) {
            if ($child->is_text) {
                $out .= $child->text;
            }
        }
        return $out;
    }

    public function count(): int
    {
        if ($this->attr_pairs !== null) {
            return count($this->attr_pairs);
        }
        return count($this->nodes);
    }

    /** @param mixed $offset */
    public function offsetExists($offset): bool
    {
        if (is_int($offset)) {
            return isset($this->nodes[$offset]);
        }
        $first = $this->nodes[0] ?? null;
        return $first !== null && isset($first->attrs[(string) $offset]);
    }

    /** @param mixed $offset */
    public function offsetGet($offset): ?SimpleXMLElement
    {
        if (is_int($offset)) {
            return isset($this->nodes[$offset]) ? self::fromNodes([$this->nodes[$offset]]) : null;
        }
        $first = $this->nodes[0] ?? null;
        if ($first === null || !isset($first->attrs[(string) $offset])) {
            return null;
        }
        return self::fromAttribute($first->attrs[(string) $offset]);
    }

    /**
     * @param mixed $offset
     * @param mixed $value
     */
    public function offsetSet($offset, $value): void
    {
        $first = $this->nodes[0] ?? null;
        if ($first !== null && !is_int($offset)) {
            $first->attrs[(string) $offset] = (string) $value;
        }
    }

    /** @param mixed $offset */
    public function offsetUnset($offset): void
    {
        $first = $this->nodes[0] ?? null;
        if ($first !== null && !is_int($offset)) {
            unset($first->attrs[(string) $offset]);
        }
    }

    public function current(): SimpleXMLElement
    {
        if ($this->attr_pairs !== null) {
            return self::fromAttribute($this->attr_pairs[$this->pos][1] ?? '');
        }
        return self::fromNodes(isset($this->nodes[$this->pos]) ? [$this->nodes[$this->pos]] : []);
    }

    public function key(): string
    {
        if ($this->attr_pairs !== null) {
            return $this->attr_pairs[$this->pos][0] ?? '';
        }
        return isset($this->nodes[$this->pos]) ? $this->nodes[$this->pos]->name : '';
    }

    public function next(): void
    {
        $this->pos++;
    }

    public function rewind(): void
    {
        $this->pos = 0;
    }

    public function valid(): bool
    {
        if ($this->attr_pairs !== null) {
            return isset($this->attr_pairs[$this->pos]);
        }
        return isset($this->nodes[$this->pos]);
    }
}

/**
 * @property string $nodeName
 * @property string $nodeValue
 * @property string $textContent
 * @property ?DOMNode $parentNode
 * @property ?DOMNode $firstChild
 * @property DOMNodeList $childNodes
 * @property ?DOMDocument $ownerDocument
 */
class DOMNode
{
    /** @internal */
    public XmlNode $xml;

    /** @internal */
    public ?DOMDocument $doc = null;

    /** @internal */
    public function __construct(XmlNode $xml, ?DOMDocument $doc)
    {
        $this->xml = $xml;
        $this->doc = $doc;
    }

    /** @internal */
    public static function wrap(XmlNode $xml, ?DOMDocument $doc): DOMNode
    {
        if ($xml->is_text) {
            return new DOMText($xml, $doc);
        }
        return new DOMElement($xml, $doc);
    }

    /** @return mixed */
    public function __get(string $name)
    {
        switch ($name) {
            case 'nodeName':
                return $this->xml->name;
            case 'nodeValue':
            case 'textContent':
                return $this->xml->textContent();
            case 'parentNode':
                return $this->xml->parent === null ? null : self::wrap($this->xml->parent, $this->doc);
            case 'firstChild':
                return isset($this->xml->children[0]) ? self::wrap($this->xml->children[0], $this->doc) : null;
            case 'childNodes':
                $nodes = [];
                foreach ($this->xml->children as $child) {
                    $nodes[] = self::wrap($child, $this->doc);
                }
                return new DOMNodeList($nodes);
            case 'ownerDocument':
                return $this->doc;
        }
        return null;
    }

    public function __isset(string $name): bool
    {
        return in_array($name, ['nodeName', 'nodeValue', 'textContent', 'parentNode', 'firstChild', 'childNodes', 'ownerDocument'], true);
    }

    public function appendChild(DOMNode $node): DOMNode
    {
        $this->xml->appendChild($node->xml);
        $node->doc = $this->doc ?? ($this instanceof DOMDocument ? $this : null);
        return $node;
    }

    public function removeChild(DOMNode $child): DOMNode
    {
        $this->xml->removeChild($child->xml);
        return $child;
    }

    public function hasChildNodes(): bool
    {
        return $this->xml->children !== [];
    }
}

class DOMText extends DOMNode
{
}

class DOMElement extends DOMNode
{
    public function getAttribute(string $qualifiedName): string
    {
        return $this->xml->attrs[$qualifiedName] ?? '';
    }

    public function hasAttribute(string $qualifiedName): bool
    {
        return isset($this->xml->attrs[$qualifiedName]);
    }

    public function setAttribute(string $qualifiedName, string $value): void
    {
        $this->xml->attrs[$qualifiedName] = $value;
    }

    public function removeAttribute(string $qualifiedName): bool
    {
        if (!isset($this->xml->attrs[$qualifiedName])) {
            return false;
        }
        unset($this->xml->attrs[$qualifiedName]);
        return true;
    }

    public function getElementsByTagName(string $qualifiedName): DOMNodeList
    {
        $nodes = [];
        foreach ($this->xml->descendantsNamed($qualifiedName) as $n) {
            $nodes[] = DOMNode::wrap($n, $this->doc);
        }
        return new DOMNodeList($nodes);
    }
}

/**
 * @property ?DOMElement $documentElement
 */
class DOMDocument extends DOMNode
{
    public bool $formatOutput = false;

    public bool $preserveWhiteSpace = true;

    public string $version;

    public string $encoding;

    public function __construct(string $version = '1.0', string $encoding = '')
    {
        parent::__construct(new XmlNode('#document'), null);
        $this->doc = $this;
        $this->version = $version;
        $this->encoding = $encoding;
    }

    /** @return mixed */
    public function __get(string $name)
    {
        if ($name === 'documentElement') {
            $els = $this->xml->elementChildren();
            return $els === [] ? null : DOMNode::wrap($els[0], $this);
        }
        return parent::__get($name);
    }

    public function __isset(string $name): bool
    {
        return $name === 'documentElement' || parent::__isset($name);
    }

    public function loadXML(string $source, int $options = 0): bool
    {
        $root = XmlNode::parse($source);
        if ($root === null) {
            return false;
        }
        $this->xml = new XmlNode('#document');
        $this->xml->appendChild($root);
        return true;
    }

    public function load(string $filename, int $options = 0): bool
    {
        $contents = file_get_contents($filename);
        if ($contents === false) {
            return false;
        }
        return $this->loadXML($contents, $options);
    }

    public function xinclude(int $options = 0): int
    {
        return 0;
    }

    public function schemaValidate(string $filename, int $flags = 0): bool
    {
        return true;
    }

    /** @return string|false */
    public function saveXML(?DOMNode $node = null, int $options = 0)
    {
        if ($node !== null && !$node instanceof DOMDocument) {
            return $node->xml->serialize($this->formatOutput);
        }
        $out = '<?xml version="' . $this->version . '"' . ($this->encoding !== '' ? ' encoding="' . $this->encoding . '"' : '') . "?>\n";
        foreach ($this->xml->children as $child) {
            $out .= $child->serialize($this->formatOutput);
        }
        if (!$this->formatOutput) {
            $out .= "\n";
        }
        return $out;
    }

    public function createElement(string $localName, string $value = ''): DOMElement
    {
        $node = new XmlNode($localName);
        if ($value !== '') {
            $node->appendChild(new XmlNode('#text', $value, true));
        }
        return new DOMElement($node, $this);
    }

    public function createTextNode(string $data): DOMText
    {
        return new DOMText(new XmlNode('#text', $data, true), $this);
    }

    public function getElementsByTagName(string $qualifiedName): DOMNodeList
    {
        $nodes = [];
        foreach ($this->xml->descendantsNamed($qualifiedName) as $n) {
            $nodes[] = DOMNode::wrap($n, $this);
        }
        return new DOMNodeList($nodes);
    }
}

/**
 * @implements IteratorAggregate<int, DOMNode>
 */
class DOMNodeList implements Countable, IteratorAggregate
{
    public int $length;

    /** @param list<DOMNode> $nodes */
    public function __construct(private array $nodes = [])
    {
        $this->length = count($nodes);
    }

    public function item(int $index): ?DOMNode
    {
        return $this->nodes[$index] ?? null;
    }

    public function count(): int
    {
        return count($this->nodes);
    }

    /** @return Iterator<int, DOMNode> */
    public function getIterator(): Iterator
    {
        foreach ($this->nodes as $i => $node) {
            yield $i => $node;
        }
    }
}

class LibXMLError
{
    public int $level = 0;
    public int $code = 0;
    public int $column = 0;
    public string $message = '';
    public string $file = '';
    public int $line = 0;
}

function simplexml_import_dom(DOMNode $node): ?SimpleXMLElement
{
    if ($node instanceof DOMDocument) {
        $els = $node->xml->elementChildren();
        return $els === [] ? null : SimpleXMLElement::fromNodes([$els[0]]);
    }
    return SimpleXMLElement::fromNodes([$node->xml]);
}

/** @return SimpleXMLElement|false */
function simplexml_load_string(string $data, ?string $class_name = SimpleXMLElement::class, int $options = 0)
{
    $root = XmlNode::parse($data);
    return $root === null ? false : SimpleXMLElement::fromNodes([$root]);
}

/** @return SimpleXMLElement|false */
function simplexml_load_file(string $filename, ?string $class_name = SimpleXMLElement::class, int $options = 0)
{
    $contents = file_get_contents($filename);
    if ($contents === false) {
        return false;
    }
    return simplexml_load_string($contents, $class_name, $options);
}

function dom_import_simplexml(SimpleXMLElement $node): DOMElement
{
    $first = $node->firstNode();
    return new DOMElement($first ?? new XmlNode('x'), null);
}

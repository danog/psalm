//! Static tables of the PHP builtin class-likes the runtime knows about. The compiled program's own
//! classes, functions and constants are static tables generated into every crate (`crate::names`),
//! which fall back to these for names of PHP builtins.

use crate::string::Str;

/// One compiled class-like in a crate's generated `names` table.
pub struct ClassInfo {
    pub name: &'static str,
    /// lowercased fully qualified names of the ancestors (parents and interfaces)
    pub ancestors: &'static [&'static str],
    /// 0 class, 1 interface, 2 trait, 3 enum
    pub kind: u8,
    /// source file the class was compiled from ("" for runtime stubs)
    pub file: &'static str,
}

/// Builtin PHP classes that exist in any interpreter even though the runtime has no code for them.
pub const BUILTIN_CLASSES: &[&str] = &[
    "stdclass", "datetime", "datetimeimmutable", "dateinterval", "dateperiod", "datetimezone", "closure", "generator",
    "arrayobject", "arrayiterator", "splobjectstorage", "splstack", "splqueue", "spldoublylinkedlist", "splfixedarray",
    "splpriorityqueue", "splminheap", "splmaxheap", "weakmap", "weakreference", "exception", "error", "errorexception",
    "typeerror", "valueerror", "arithmeticerror", "divisionbyzeroerror", "argumentcounterror", "runtimeexception",
    "logicexception", "invalidargumentexception", "domainexception", "lengthexception", "outofrangeexception",
    "outofboundsexception", "rangeexception", "overflowexception", "underflowexception", "unexpectedvalueexception",
    "reflectionclass", "reflectionmethod", "reflectionproperty", "reflectionfunction", "reflectionnamedtype",
    "simplexmlelement", "domdocument", "domelement", "domnode", "pdo", "mysqli", "curlhandle", "attribute",
];
pub const BUILTIN_INTERFACES: &[&str] = &[
    "traversable", "iterator", "iteratoraggregate", "arrayaccess", "countable", "stringable", "throwable",
    "jsonserializable", "serializable", "unitenum", "backedenum", "datetimeinterface", "outeriterator",
    "recursiveiterator", "seekableiterator", "splobserver", "splsubject",
];

/// Lowercased class name without a leading backslash: the key of every name table.
pub fn norm(name: &Str) -> Vec<u8> {
    let b = name.as_bytes();
    let b = if b.first() == Some(&b'\\') { &b[1..] } else { b };
    b.to_ascii_lowercase()
}

pub fn builtin_class_exists(lc: &[u8]) -> bool {
    BUILTIN_CLASSES.iter().any(|b| b.as_bytes() == lc)
}

pub fn builtin_interface_exists(lc: &[u8]) -> bool {
    BUILTIN_INTERFACES.iter().any(|b| b.as_bytes() == lc)
}

/// Names of the builtin classes (or interfaces), in PHP's spelling.
pub fn builtin_declared(interfaces: bool) -> crate::list::List<Str> {
    let mut out = crate::list::List::new();
    let builtin = if interfaces { BUILTIN_INTERFACES } else { BUILTIN_CLASSES };
    for b in builtin {
        out.push(Str::from_str(&canonical_builtin_name(b)));
    }
    out
}

/// PHP's spelling of a builtin class name known only in lowercase.
fn canonical_builtin_name(lc: &str) -> String {
    match lc {
        "stdclass" => "stdClass".into(),
        "datetime" => "DateTime".into(),
        "datetimeimmutable" => "DateTimeImmutable".into(),
        "datetimeinterface" => "DateTimeInterface".into(),
        "dateinterval" => "DateInterval".into(),
        "dateperiod" => "DatePeriod".into(),
        "datetimezone" => "DateTimeZone".into(),
        "arrayobject" => "ArrayObject".into(),
        "arrayiterator" => "ArrayIterator".into(),
        "arrayaccess" => "ArrayAccess".into(),
        "iteratoraggregate" => "IteratorAggregate".into(),
        "jsonserializable" => "JsonSerializable".into(),
        "splobjectstorage" => "SplObjectStorage".into(),
        "weakmap" => "WeakMap".into(),
        "weakreference" => "WeakReference".into(),
        "unitenum" => "UnitEnum".into(),
        "backedenum" => "BackedEnum".into(),
        other => {
            let mut s = other.to_string();
            if let Some(f) = s.get_mut(0..1) {
                f.make_ascii_uppercase();
            }
            s
        }
    }
}

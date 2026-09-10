//! PHP constants.

use crate::containers::Resource;
use crate::mixed::Mixed;
use crate::string::Str;
use std::rc::Rc;

pub const PHP_EOL: Str = Str::from_static("\n");
pub const DIRECTORY_SEPARATOR: Str = Str::from_static("/");
pub const PATH_SEPARATOR: Str = Str::from_static(":");
pub const PHP_OS: Str = Str::from_static("Linux");
pub const PHP_OS_FAMILY: Str = Str::from_static("Linux");
pub const PHP_VERSION: Str = Str::from_static("8.5.0");
pub const PHP_VERSION_ID: i64 = 80500;
pub const PHP_MAJOR_VERSION: i64 = 8;
pub const PHP_MINOR_VERSION: i64 = 5;
pub const PHP_RELEASE_VERSION: i64 = 0;
pub const PHP_INT_MAX: i64 = i64::MAX;
pub const PHP_INT_MIN: i64 = i64::MIN;
pub const PHP_INT_SIZE: i64 = 8;
pub const PHP_FLOAT_EPSILON: f64 = f64::EPSILON;
pub const PHP_FLOAT_MAX: f64 = f64::MAX;
pub const PHP_FLOAT_MIN: f64 = f64::MIN_POSITIVE;
pub const PHP_FLOAT_DIG: i64 = 15;
pub const NAN: f64 = f64::NAN;
pub const INF: f64 = f64::INFINITY;
pub const M_PI: f64 = std::f64::consts::PI;
pub const PHP_BINARY: Str = Str::from_static("/usr/bin/php");
pub const PHP_SAPI: Str = Str::from_static("cli");
pub const PHP_MAXPATHLEN: i64 = 4096;
pub const PSALM_VERSION: Str = Str::from_static("6.0.0-rust");
pub const PHP_PARSER_VERSION: Str = Str::from_static("5.8.0");

pub const E_ERROR: i64 = 1;
pub const E_WARNING: i64 = 2;
pub const E_PARSE: i64 = 4;
pub const E_NOTICE: i64 = 8;
pub const E_CORE_ERROR: i64 = 16;
pub const E_CORE_WARNING: i64 = 32;
pub const E_COMPILE_ERROR: i64 = 64;
pub const E_COMPILE_WARNING: i64 = 128;
pub const E_USER_ERROR: i64 = 256;
pub const E_USER_WARNING: i64 = 512;
pub const E_USER_NOTICE: i64 = 1024;
pub const E_STRICT: i64 = 2048;
pub const E_RECOVERABLE_ERROR: i64 = 4096;
pub const E_DEPRECATED: i64 = 8192;
pub const E_USER_DEPRECATED: i64 = 16384;
pub const E_ALL: i64 = 32767;

pub const SORT_REGULAR: i64 = 0;
pub const SORT_NUMERIC: i64 = 1;
pub const SORT_STRING: i64 = 2;
pub const SORT_DESC: i64 = 3;
pub const SORT_ASC: i64 = 4;
pub const SORT_NATURAL: i64 = 6;
pub const SORT_FLAG_CASE: i64 = 8;
pub const COUNT_NORMAL: i64 = 0;
pub const COUNT_RECURSIVE: i64 = 1;
pub const ARRAY_FILTER_USE_KEY: i64 = 2;
pub const ARRAY_FILTER_USE_BOTH: i64 = 1;
pub const STR_PAD_LEFT: i64 = 0;
pub const STR_PAD_RIGHT: i64 = 1;
pub const STR_PAD_BOTH: i64 = 2;

pub const JSON_HEX_TAG: i64 = 1;
pub const JSON_HEX_AMP: i64 = 2;
pub const JSON_HEX_APOS: i64 = 4;
pub const JSON_HEX_QUOT: i64 = 8;
pub const JSON_FORCE_OBJECT: i64 = 16;
pub const JSON_NUMERIC_CHECK: i64 = 32;
pub const JSON_UNESCAPED_SLASHES: i64 = 64;
pub const JSON_PRETTY_PRINT: i64 = 128;
pub const JSON_UNESCAPED_UNICODE: i64 = 256;
pub const JSON_PARTIAL_OUTPUT_ON_ERROR: i64 = 512;
pub const JSON_PRESERVE_ZERO_FRACTION: i64 = 1024;
pub const JSON_OBJECT_AS_ARRAY: i64 = 1;
pub const JSON_BIGINT_AS_STRING: i64 = 2;
pub const JSON_INVALID_UTF8_IGNORE: i64 = 1048576;
pub const JSON_INVALID_UTF8_SUBSTITUTE: i64 = 2097152;
pub const JSON_THROW_ON_ERROR: i64 = 4194304;
pub const JSON_ERROR_NONE: i64 = 0;
pub const JSON_ERROR_SYNTAX: i64 = 4;

pub const PREG_PATTERN_ORDER: i64 = 1;
pub const PREG_SET_ORDER: i64 = 2;
pub const PREG_OFFSET_CAPTURE: i64 = 256;
pub const PREG_UNMATCHED_AS_NULL: i64 = 512;
pub const PREG_SPLIT_NO_EMPTY: i64 = 1;
pub const PREG_SPLIT_DELIM_CAPTURE: i64 = 2;
pub const PREG_SPLIT_OFFSET_CAPTURE: i64 = 4;
pub const PREG_NO_ERROR: i64 = 0;

pub const ENT_HTML401: i64 = 0;
pub const ENT_COMPAT: i64 = 2;
pub const ENT_QUOTES: i64 = 3;
pub const ENT_NOQUOTES: i64 = 0;
pub const ENT_HTML5: i64 = 48;
pub const ENT_SUBSTITUTE: i64 = 8;

pub const LOCK_SH: i64 = 1;
pub const LOCK_EX: i64 = 2;
pub const LOCK_UN: i64 = 3;
pub const LOCK_NB: i64 = 4;
pub const FILE_IGNORE_NEW_LINES: i64 = 2;
pub const FILE_SKIP_EMPTY_LINES: i64 = 4;
pub const FILE_APPEND: i64 = 8;
pub const GLOB_MARK: i64 = 2;
pub const GLOB_NOSORT: i64 = 4;
pub const GLOB_BRACE: i64 = 1024;
pub const GLOB_ONLYDIR: i64 = 8192;
pub const SCANDIR_SORT_ASCENDING: i64 = 0;
pub const SEEK_SET: i64 = 0;
pub const SEEK_CUR: i64 = 1;
pub const SEEK_END: i64 = 2;

pub const PHP_ROUND_HALF_UP: i64 = 1;
pub const PHP_ROUND_HALF_DOWN: i64 = 2;
pub const PHP_ROUND_HALF_EVEN: i64 = 3;
pub const PHP_ROUND_HALF_ODD: i64 = 4;
pub const EXTR_OVERWRITE: i64 = 0;
pub const EXTR_SKIP: i64 = 1;
pub const CASE_LOWER: i64 = 0;
pub const CASE_UPPER: i64 = 1;
pub const LC_CTYPE: i64 = 0;
pub const LC_ALL: i64 = 6;
pub const FILTER_VALIDATE_INT: i64 = 257;
pub const FILTER_VALIDATE_BOOLEAN: i64 = 258;
pub const FILTER_VALIDATE_BOOL: i64 = 258;
pub const FILTER_VALIDATE_FLOAT: i64 = 259;
pub const FILTER_VALIDATE_URL: i64 = 273;
pub const FILTER_VALIDATE_EMAIL: i64 = 274;
pub const FILTER_DEFAULT: i64 = 516;
pub const FILTER_NULL_ON_FAILURE: i64 = 134217728;
pub const FILTER_FLAG_ALLOW_OCTAL: i64 = 1;
pub const FILTER_FLAG_ALLOW_HEX: i64 = 2;
pub const PHP_URL_SCHEME: i64 = 0;
pub const PHP_URL_HOST: i64 = 1;
pub const PHP_URL_PATH: i64 = 5;
pub const MB_CASE_UPPER: i64 = 0;
pub const MB_CASE_LOWER: i64 = 1;
pub const MB_CASE_TITLE: i64 = 2;
pub const DEBUG_BACKTRACE_IGNORE_ARGS: i64 = 2;
pub const PHP_OUTPUT_HANDLER_STDFLAGS: i64 = 112;
pub const CURLOPT_URL: i64 = 10002;
pub const LIBXML_NOENT: i64 = 2;
pub const LIBXML_COMPACT: i64 = 65536;
pub const LIBXML_NOEMPTYTAG: i64 = 4;
pub const LIBXML_NOCDATA: i64 = 16384;
pub const LIBXML_NOBLANKS: i64 = 256;
pub const LIBXML_NOERROR: i64 = 32;
pub const LIBXML_NOWARNING: i64 = 64;
pub const LIBXML_NONET: i64 = 2048;
pub const LIBXML_ERR_WARNING: i64 = 1;
pub const LIBXML_ERR_ERROR: i64 = 2;
pub const LIBXML_ERR_FATAL: i64 = 3;
pub const XML_ELEMENT_NODE: i64 = 1;
pub const XML_TEXT_NODE: i64 = 3;

pub fn STDIN() -> Rc<Resource> {
    crate::containers::stdin_res()
}
pub fn STDOUT() -> Rc<Resource> {
    crate::containers::stdout_res()
}
pub fn STDERR() -> Rc<Resource> {
    crate::containers::stderr_res()
}

// ---------------------------------------------------------------- tokenizer constants (PHP 8.4 values)

macro_rules! tokens {
    ($($name:ident = $val:expr),* $(,)?) => {
        $(pub const $name: i64 = $val;)*
        pub static TOKEN_NAMES: &[(&str, i64)] = &[$((stringify!($name), $val)),*];
    };
}

tokens! {
    T_LNUMBER = 260, T_DNUMBER = 261, T_STRING = 262, T_NAME_FULLY_QUALIFIED = 263, T_NAME_RELATIVE = 264,
    T_NAME_QUALIFIED = 265, T_VARIABLE = 266, T_INLINE_HTML = 267, T_ENCAPSED_AND_WHITESPACE = 268,
    T_CONSTANT_ENCAPSED_STRING = 269, T_STRING_VARNAME = 270, T_NUM_STRING = 271, T_INCLUDE = 272,
    T_INCLUDE_ONCE = 273, T_EVAL = 274, T_REQUIRE = 275, T_REQUIRE_ONCE = 276, T_LOGICAL_OR = 277,
    T_LOGICAL_XOR = 278, T_LOGICAL_AND = 279, T_PRINT = 280, T_YIELD = 281, T_YIELD_FROM = 282,
    T_INSTANCEOF = 283, T_NEW = 284, T_CLONE = 285, T_EXIT = 286, T_IF = 287, T_ELSEIF = 288, T_ELSE = 289,
    T_ENDIF = 290, T_ECHO = 291, T_DO = 292, T_WHILE = 293, T_ENDWHILE = 294, T_FOR = 295, T_ENDFOR = 296,
    T_FOREACH = 297, T_ENDFOREACH = 298, T_DECLARE = 299, T_ENDDECLARE = 300, T_AS = 301, T_SWITCH = 302,
    T_ENDSWITCH = 303, T_CASE = 304, T_DEFAULT = 305, T_MATCH = 306, T_BREAK = 307, T_CONTINUE = 308,
    T_GOTO = 309, T_FUNCTION = 310, T_FN = 311, T_CONST = 312, T_RETURN = 313, T_TRY = 314, T_CATCH = 315,
    T_FINALLY = 316, T_THROW = 317, T_USE = 318, T_INSTEADOF = 319, T_GLOBAL = 320, T_STATIC = 321,
    T_ABSTRACT = 322, T_FINAL = 323, T_PRIVATE = 324, T_PROTECTED = 325, T_PUBLIC = 326, T_PRIVATE_SET = 327,
    T_PROTECTED_SET = 328, T_PUBLIC_SET = 329, T_READONLY = 330, T_VAR = 331, T_UNSET = 332, T_ISSET = 333,
    T_EMPTY = 334, T_HALT_COMPILER = 335, T_CLASS = 336, T_TRAIT = 337, T_INTERFACE = 338, T_ENUM = 339,
    T_EXTENDS = 340, T_IMPLEMENTS = 341, T_NAMESPACE = 342, T_LIST = 343, T_ARRAY = 344, T_CALLABLE = 345,
    T_LINE = 346, T_FILE = 347, T_DIR = 348, T_CLASS_C = 349, T_TRAIT_C = 350, T_METHOD_C = 351,
    T_FUNC_C = 352, T_PROPERTY_C = 353, T_NS_C = 354, T_ATTRIBUTE = 355, T_PLUS_EQUAL = 356,
    T_MINUS_EQUAL = 357, T_MUL_EQUAL = 358, T_DIV_EQUAL = 359, T_CONCAT_EQUAL = 360, T_MOD_EQUAL = 361,
    T_AND_EQUAL = 362, T_OR_EQUAL = 363, T_XOR_EQUAL = 364, T_SL_EQUAL = 365, T_SR_EQUAL = 366,
    T_COALESCE_EQUAL = 367, T_BOOLEAN_OR = 368, T_BOOLEAN_AND = 369, T_IS_EQUAL = 370, T_IS_NOT_EQUAL = 371,
    T_IS_IDENTICAL = 372, T_IS_NOT_IDENTICAL = 373, T_IS_SMALLER_OR_EQUAL = 374, T_IS_GREATER_OR_EQUAL = 375,
    T_SPACESHIP = 376, T_SL = 377, T_SR = 378, T_INC = 379, T_DEC = 380, T_INT_CAST = 381,
    T_DOUBLE_CAST = 382, T_STRING_CAST = 383, T_ARRAY_CAST = 384, T_OBJECT_CAST = 385, T_BOOL_CAST = 386,
    T_UNSET_CAST = 387, T_VOID_CAST = 388, T_OBJECT_OPERATOR = 389, T_NULLSAFE_OBJECT_OPERATOR = 390,
    T_DOUBLE_ARROW = 391, T_COMMENT = 392, T_DOC_COMMENT = 393, T_OPEN_TAG = 394, T_OPEN_TAG_WITH_ECHO = 395,
    T_CLOSE_TAG = 396, T_WHITESPACE = 397, T_START_HEREDOC = 398, T_END_HEREDOC = 399,
    T_DOLLAR_OPEN_CURLY_BRACES = 400, T_CURLY_OPEN = 401, T_PAAMAYIM_NEKUDOTAYIM = 402, T_NS_SEPARATOR = 403,
    T_ELLIPSIS = 404, T_COALESCE = 405, T_POW = 406, T_POW_EQUAL = 407, T_PIPE = 408,
    T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG = 409, T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG = 410,
    T_BAD_CHARACTER = 411, T_DOUBLE_COLON = 402,
}

/// Value of a `T_*` token constant by name.
pub fn token_value(name: &[u8]) -> Option<i64> {
    TOKEN_NAMES.iter().find(|(n, _)| n.as_bytes() == name).map(|(_, v)| *v)
}

pub fn token_name(id: i64) -> Str {
    for (name, v) in TOKEN_NAMES {
        if *v == id && *name != "T_PAAMAYIM_NEKUDOTAYIM" {
            return Str::from_static(name);
        }
    }
    Str::from_static("UNKNOWN")
}

pub fn builtin_defined(name: &[u8]) -> bool {
    builtin_value(name).is_some()
}

pub fn builtin_value(name: &[u8]) -> Option<Mixed> {
    let n = std::str::from_utf8(name).ok()?;
    for (tname, v) in TOKEN_NAMES {
        if *tname == n {
            return Some(Mixed::Int(*v));
        }
    }
    Some(match n {
        "PHP_EOL" => Mixed::Str(PHP_EOL),
        "PHP_VERSION" => Mixed::Str(PHP_VERSION),
        "PHP_VERSION_ID" => Mixed::Int(PHP_VERSION_ID),
        "PHP_INT_MAX" => Mixed::Int(PHP_INT_MAX),
        "PHP_INT_MIN" => Mixed::Int(PHP_INT_MIN),
        "PHP_INT_SIZE" => Mixed::Int(PHP_INT_SIZE),
        "PHP_OS" | "PHP_OS_FAMILY" => Mixed::Str(PHP_OS),
        "DIRECTORY_SEPARATOR" => Mixed::Str(DIRECTORY_SEPARATOR),
        "E_ALL" => Mixed::Int(E_ALL),
        "E_STRICT" => Mixed::Int(E_STRICT),
        "PSALM_COMPILED" => Mixed::Bool(true),
        "PSALM_VERSION" => Mixed::Str(PSALM_VERSION),
        "PHP_PARSER_VERSION" => Mixed::Str(PHP_PARSER_VERSION),
        _ => return None,
    })
}

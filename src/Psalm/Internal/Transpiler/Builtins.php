<?php

declare(strict_types=1);

namespace Psalm\Internal\Transpiler;

use PhpParser\Node\Arg;
use PhpParser\Node\Expr;

use function array_map;
use function count;
use function implode;
use function in_array;
use function strtolower;

/**
 * Emission rules for PHP builtin functions, mapped onto the Rust runtime.
 *
 * @internal
 */
final class Builtins
{
    /** Constants provided by `php_rt::consts`, with their Rust types. */
    private const CONSTANTS = [
        'PHP_EOL' => 'str', 'DIRECTORY_SEPARATOR' => 'str', 'PATH_SEPARATOR' => 'str', 'PHP_OS' => 'str', 'PHP_OS_FAMILY' => 'str',
        'PHP_VERSION' => 'str', 'PHP_VERSION_ID' => 'int', 'PHP_MAJOR_VERSION' => 'int', 'PHP_MINOR_VERSION' => 'int', 'PHP_RELEASE_VERSION' => 'int',
        'PHP_INT_MAX' => 'int', 'PHP_INT_MIN' => 'int', 'PHP_INT_SIZE' => 'int', 'PHP_FLOAT_EPSILON' => 'float', 'PHP_FLOAT_MAX' => 'float', 'PHP_FLOAT_MIN' => 'float', 'PHP_FLOAT_DIG' => 'int',
        'NAN' => 'float', 'INF' => 'float', 'M_PI' => 'float', 'PHP_BINARY' => 'str', 'PHP_SAPI' => 'str', 'PHP_MAXPATHLEN' => 'int',
        'E_ALL' => 'int', 'E_ERROR' => 'int', 'E_WARNING' => 'int', 'E_NOTICE' => 'int', 'E_STRICT' => 'int', 'E_DEPRECATED' => 'int', 'E_USER_ERROR' => 'int', 'E_USER_WARNING' => 'int', 'E_USER_NOTICE' => 'int', 'E_USER_DEPRECATED' => 'int', 'E_PARSE' => 'int', 'E_CORE_ERROR' => 'int', 'E_COMPILE_ERROR' => 'int', 'E_RECOVERABLE_ERROR' => 'int', 'E_CORE_WARNING' => 'int', 'E_COMPILE_WARNING' => 'int',
        'SORT_REGULAR' => 'int', 'SORT_NUMERIC' => 'int', 'SORT_STRING' => 'int', 'SORT_FLAG_CASE' => 'int', 'SORT_NATURAL' => 'int', 'SORT_ASC' => 'int', 'SORT_DESC' => 'int',
        'COUNT_RECURSIVE' => 'int', 'COUNT_NORMAL' => 'int', 'ARRAY_FILTER_USE_KEY' => 'int', 'ARRAY_FILTER_USE_BOTH' => 'int',
        'STR_PAD_LEFT' => 'int', 'STR_PAD_RIGHT' => 'int', 'STR_PAD_BOTH' => 'int',
        'JSON_PRETTY_PRINT' => 'int', 'JSON_UNESCAPED_SLASHES' => 'int', 'JSON_UNESCAPED_UNICODE' => 'int', 'JSON_THROW_ON_ERROR' => 'int', 'JSON_ERROR_NONE' => 'int', 'JSON_OBJECT_AS_ARRAY' => 'int', 'JSON_PRESERVE_ZERO_FRACTION' => 'int', 'JSON_HEX_TAG' => 'int', 'JSON_HEX_AMP' => 'int', 'JSON_HEX_APOS' => 'int', 'JSON_HEX_QUOT' => 'int', 'JSON_INVALID_UTF8_SUBSTITUTE' => 'int', 'JSON_INVALID_UTF8_IGNORE' => 'int', 'JSON_PARTIAL_OUTPUT_ON_ERROR' => 'int', 'JSON_ERROR_SYNTAX' => 'int', 'JSON_FORCE_OBJECT' => 'int', 'JSON_NUMERIC_CHECK' => 'int', 'JSON_BIGINT_AS_STRING' => 'int',
        'PREG_SPLIT_NO_EMPTY' => 'int', 'PREG_SPLIT_DELIM_CAPTURE' => 'int', 'PREG_SPLIT_OFFSET_CAPTURE' => 'int', 'PREG_OFFSET_CAPTURE' => 'int', 'PREG_UNMATCHED_AS_NULL' => 'int', 'PREG_SET_ORDER' => 'int', 'PREG_PATTERN_ORDER' => 'int', 'PREG_NO_ERROR' => 'int',
        'ENT_QUOTES' => 'int', 'ENT_HTML5' => 'int', 'ENT_COMPAT' => 'int', 'ENT_NOQUOTES' => 'int', 'ENT_HTML401' => 'int', 'ENT_SUBSTITUTE' => 'int',
        'LOCK_EX' => 'int', 'LOCK_SH' => 'int', 'LOCK_UN' => 'int', 'LOCK_NB' => 'int', 'FILE_APPEND' => 'int', 'FILE_IGNORE_NEW_LINES' => 'int', 'FILE_SKIP_EMPTY_LINES' => 'int', 'GLOB_ONLYDIR' => 'int', 'GLOB_BRACE' => 'int', 'GLOB_MARK' => 'int', 'GLOB_NOSORT' => 'int', 'SCANDIR_SORT_ASCENDING' => 'int',
        'STDIN' => 'resource', 'STDOUT' => 'resource', 'STDERR' => 'resource',
        'PHP_ROUND_HALF_UP' => 'int', 'PHP_ROUND_HALF_DOWN' => 'int', 'PHP_ROUND_HALF_EVEN' => 'int', 'PHP_ROUND_HALF_ODD' => 'int',
        'EXTR_OVERWRITE' => 'int', 'EXTR_SKIP' => 'int', 'CASE_LOWER' => 'int', 'CASE_UPPER' => 'int', 'LC_ALL' => 'int', 'LC_CTYPE' => 'int',
        'FILTER_VALIDATE_INT' => 'int', 'FILTER_VALIDATE_FLOAT' => 'int', 'FILTER_VALIDATE_BOOLEAN' => 'int', 'FILTER_VALIDATE_BOOL' => 'int', 'FILTER_NULL_ON_FAILURE' => 'int', 'FILTER_VALIDATE_URL' => 'int', 'FILTER_VALIDATE_EMAIL' => 'int', 'FILTER_DEFAULT' => 'int', 'FILTER_FLAG_ALLOW_OCTAL' => 'int', 'FILTER_FLAG_ALLOW_HEX' => 'int',
        'SEEK_SET' => 'int', 'SEEK_CUR' => 'int', 'SEEK_END' => 'int', 'PHP_URL_PATH' => 'int', 'PHP_URL_SCHEME' => 'int', 'PHP_URL_HOST' => 'int',
        'MB_CASE_TITLE' => 'int', 'MB_CASE_UPPER' => 'int', 'MB_CASE_LOWER' => 'int',
        'DEBUG_BACKTRACE_IGNORE_ARGS' => 'int', 'PHP_OUTPUT_HANDLER_STDFLAGS' => 'int', 'CURLOPT_URL' => 'int',
        'LIBXML_NONET' => 'int', 'LIBXML_NOBLANKS' => 'int', 'LIBXML_NOENT' => 'int', 'LIBXML_NOERROR' => 'int', 'LIBXML_NOWARNING' => 'int', 'LIBXML_ERR_ERROR' => 'int', 'LIBXML_ERR_WARNING' => 'int', 'LIBXML_ERR_FATAL' => 'int',
        'XML_ELEMENT_NODE' => 'int', 'XML_TEXT_NODE' => 'int',
        'PSALM_VERSION' => 'str', 'PHP_PARSER_VERSION' => 'str',
    ];

    /** Tokenizer T_* constants exist as `consts::T_*` in the runtime. */
    public function hasConstant(string $name): bool
    {
        return isset(self::CONSTANTS[$name]) || str_starts_with($name, 'T_');
    }

    public function constantType(string $name): RustType
    {
        $k = self::CONSTANTS[$name] ?? 'int';
        return match ($k) {
            'str' => RustType::str(),
            'float' => RustType::float(),
            'resource' => RustType::resource(),
            default => RustType::int(),
        };
    }

    /**
     * Emit a call to a builtin, or null if unknown.
     *
     * @param list<Arg> $args
     */
    public function emit(BodyEmitter $b, Expr\FuncCall $call, string $name, array $args): ?Val
    {
        $method = 'f_' . $name;
        if (method_exists($this, $method)) {
            return $this->$method($b, $call, $args);
        }
        if (isset(self::SIMPLE[$name])) {
            return $this->simple($b, $call, $args, self::SIMPLE[$name]);
        }
        return null;
    }

    /**
     * Simple mapping: [rust fn, [param specs], return type].
     * Param spec: type letter, prefix '&' passes by reference, '?' optional (Option), suffix '=' with default code.
     * Type letters: s Str, i int, f float, b bool, m Mixed, k ArrayKey, L list<mixed>, x any (natural), r resource
     * Return: same letters plus 'o<letter>' for Option, 'ls' list of str, 'ms' map str=>str, 'S' Result (adds ?)
     */
    private const SIMPLE = [
        'strlen' => ['strlen', ['&s'], 'i'],
        'strtolower' => ['strtolower', ['&s'], 's'],
        'strtoupper' => ['strtoupper', ['&s'], 's'],
        'ucfirst' => ['ucfirst', ['&s'], 's'],
        'lcfirst' => ['lcfirst', ['&s'], 's'],
        'ucwords' => ['ucwords', ['&s'], 's'],
        'strrev' => ['strrev', ['&s'], 's'],
        'substr' => ['substr', ['&s', 'i', '?i'], 's'],
        'mb_substr' => ['mb_substr', ['&s', 'i', '?i'], 's'],
        'mb_strcut' => ['mb_strcut', ['&s', 'i', '?i'], 's'],
        'substr_count' => ['substr_count', ['&s', '&s'], 'i'],
        'strpos' => ['strpos', ['&s', '&s', 'i=0'], 'oi'],
        'stripos' => ['stripos', ['&s', '&s', 'i=0'], 'oi'],
        'strrpos' => ['strrpos', ['&s', '&s', 'i=0'], 'oi'],
        'strripos' => ['strripos', ['&s', '&s', 'i=0'], 'oi'],
        'mb_strpos' => ['mb_strpos', ['&s', '&s', 'i=0'], 'oi'],
        'mb_strlen' => ['mb_strlen', ['&s'], 'i'],
        'mb_strtolower' => ['mb_strtolower', ['&s'], 's'],
        'mb_strtoupper' => ['mb_strtoupper', ['&s'], 's'],
        'mb_str_split' => ['mb_str_split', ['&s'], 'ls'],
        'str_starts_with' => ['str_starts_with', ['&s', '&s'], 'b'],
        'str_ends_with' => ['str_ends_with', ['&s', '&s'], 'b'],
        'str_contains' => ['str_contains', ['&s', '&s'], 'b'],
        'str_repeat' => ['str_repeat', ['&s', 'i'], 's'],
        'strrchr' => ['strrchr', ['&s', '&s'], 'os'],
        'trim' => ['trim', ['&s', '?&s'], 's'],
        'ltrim' => ['ltrim', ['&s', '?&s'], 's'],
        'rtrim' => ['rtrim', ['&s', '?&s'], 's'],
        'chop' => ['rtrim', ['&s', '?&s'], 's'],
        'str_split' => ['str_split', ['&s', 'i=1'], 'ls'],
        'str_pad' => ['str_pad', ['&s', 'i', '&s= ', 'i=1'], 's'],
        'ord' => ['ord', ['&s'], 'i'],
        'chr' => ['chr', ['i'], 's'],
        'strcmp' => ['strcmp', ['&s', '&s'], 'i'],
        'strcasecmp' => ['strcasecmp', ['&s', '&s'], 'i'],
        'strncmp' => ['strncmp', ['&s', '&s', 'i'], 'i'],
        'strncasecmp' => ['strncasecmp', ['&s', '&s', 'i'], 'i'],
        'strspn' => ['strspn', ['&s', '&s'], 'i'],
        'strcspn' => ['strcspn', ['&s', '&s'], 'i'],
        'strpbrk' => ['strpbrk', ['&s', '&s'], 'os'],
        'wordwrap' => ['wordwrap', ['&s', 'i=75', '&s=\n', 'b=false'], 's'],
        'addslashes' => ['addslashes', ['&s'], 's'],
        'stripslashes' => ['stripslashes', ['&s'], 's'],
        'addcslashes' => ['addcslashes', ['&s', '&s'], 's'],
        'htmlspecialchars' => ['htmlspecialchars', ['&s'], 's'],
        'strip_tags' => ['strip_tags', ['&s'], 's'],
        'nl2br' => ['nl2br', ['&s'], 's'],
        'bin2hex' => ['bin2hex', ['&s'], 's'],
        'dechex' => ['dechex', ['i'], 's'],
        'decbin' => ['decbin', ['i'], 's'],
        'decoct' => ['decoct', ['i'], 's'],
        'hexdec' => ['hexdec', ['&s'], 'i'],
        'octdec' => ['octdec', ['&s'], 'i'],
        'bindec' => ['bindec', ['&s'], 'i'],
        'base_convert' => ['base_convert', ['&s', 'i', 'i'], 's'],
        'number_format' => ['number_format', ['f', 'i=0', '&s=.', '&s=,'], 's'],
        'md5' => ['md5', ['&s'], 's'],
        'sha1' => ['sha1', ['&s'], 's'],
        'crc32' => ['crc32', ['&s'], 'i'],
        'hash' => ['hash', ['&s', '&s'], 'Ss'],
        'urlencode' => ['urlencode', ['&s'], 's'],
        'rawurlencode' => ['rawurlencode', ['&s'], 's'],
        'urldecode' => ['urldecode', ['&s'], 's'],
        'escapeshellarg' => ['escapeshellarg', ['&s'], 's'],
        'levenshtein' => ['levenshtein', ['&s', '&s'], 'i'],
        'similar_text' => ['similar_text', ['&s', '&s'], 'i'],
        'ctype_digit' => ['ctype_digit', ['&s'], 'b'],
        'ctype_alpha' => ['ctype_alpha', ['&s'], 'b'],
        'ctype_alnum' => ['ctype_alnum', ['&s'], 'b'],
        'ctype_upper' => ['ctype_upper', ['&s'], 'b'],
        'ctype_lower' => ['ctype_lower', ['&s'], 'b'],
        'ctype_space' => ['ctype_space', ['&s'], 'b'],
        'ctype_punct' => ['ctype_punct', ['&s'], 'b'],
        'ctype_xdigit' => ['ctype_xdigit', ['&s'], 'b'],
        'uniqid' => ['uniqid', [], 's'],
        'strtok' => ['strtok', ['&s', '&s'], 'os'],
        'dirname' => ['php_dirname', ['&s', 'i=1'], 's'],
        'basename' => ['php_basename', ['&s', '?&s'], 's'],
        'pathinfo' => ['pathinfo', ['&s'], 'mss'],
        'version_compare' => ['version_compare', ['&s', '&s'], 'i'],
        'abs' => ['abs_num', ['n'], 'n'],
        'ceil' => ['ceil', ['f'], 'f'],
        'floor' => ['floor', ['f'], 'f'],
        'round' => ['round', ['f', 'i=0'], 'f'],
        'sqrt' => ['sqrt', ['f'], 'f'],
        'log' => ['log', ['f'], 'f'],
        'log10' => ['log10', ['f'], 'f'],
        'exp' => ['exp', ['f'], 'f'],
        'is_nan' => ['is_nan', ['f'], 'b'],
        'is_finite' => ['is_finite', ['f'], 'b'],
        'is_infinite' => ['is_infinite', ['f'], 'b'],
        'intdiv' => ['intdiv', ['i', 'i'], 'Si'],
        'fdiv' => ['fdiv', ['f', 'f'], 'f'],
        'fmod' => ['fmod', ['f', 'f'], 'f'],
        'mt_rand' => ['mt_rand', ['i=0', 'i=2147483647'], 'i'],
        'rand' => ['rand', ['i=0', 'i=2147483647'], 'i'],
        'random_int' => ['random_int', ['i', 'i'], 'i'],
        'mt_srand' => ['mt_srand', ['i=0'], 'u'],
        'mt_getrandmax' => ['mt_getrandmax', [], 'i'],
        'microtime' => ['microtime', ['b=false'], 'f'],
        'time' => ['time', [], 'i'],
        'hrtime' => ['hrtime_ns', ['b=false'], 'i'],
        'var_export' => ['var_export', ['&m', 'b=false'], 's'],
        'print_r' => ['print_r', ['&m', 'b=false'], 's'],
        'var_dump' => ['var_dump', ['&m'], 's'],
        'gettype' => ['gettype', ['&m'], 's'],
        'get_debug_type' => ['get_debug_type', ['&m'], 's'],
        'intval' => ['intval', ['&s', 'i=10'], 'i'],
        'floatval' => ['to_float', ['&x'], 'f'],
        'doubleval' => ['to_float', ['&x'], 'f'],
        'strval' => ['to_str', ['&x'], 's'],
        'boolval' => ['truthy', ['&x'], 'b'],
        'is_numeric' => ['is_numeric_val', ['&x'], 'b'],
        'ob_start' => ['ob_start', [], 'b'],
        'ob_get_clean' => ['ob_get_clean', [], 'os'],
        'ob_get_contents' => ['ob_get_contents', [], 'os'],
        'ob_end_clean' => ['ob_end_clean', [], 'b'],
        'ob_end_flush' => ['ob_end_flush', [], 'b'],
        'ob_get_level' => ['ob_get_level', [], 'i'],
        'file_get_contents' => ['file_get_contents', ['&s'], 'os'],
        'file_put_contents' => ['file_put_contents', ['&s', '&s', 'i=0'], 'oi'],
        'file_exists' => ['file_exists', ['&s'], 'b'],
        'is_dir' => ['is_dir', ['&s'], 'b'],
        'is_file' => ['is_file', ['&s'], 'b'],
        'is_readable' => ['is_readable', ['&s'], 'b'],
        'is_writable' => ['is_writable', ['&s'], 'b'],
        'is_link' => ['is_link', ['&s'], 'b'],
        'realpath' => ['realpath', ['&s'], 'os'],
        'getcwd' => ['getcwd', [], 's'],
        'chdir' => ['chdir', ['&s'], 'b'],
        'mkdir' => ['mkdir', ['&s', 'i=0777', 'b=false'], 'b'],
        'rmdir' => ['rmdir', ['&s'], 'b'],
        'unlink' => ['unlink', ['&s'], 'b'],
        'rename' => ['rename', ['&s', '&s'], 'b'],
        'copy' => ['copy', ['&s', '&s'], 'b'],
        'touch' => ['touch', ['&s'], 'b'],
        'filemtime' => ['filemtime', ['&s'], 'oi'],
        'filesize' => ['filesize', ['&s'], 'oi'],
        'tempnam' => ['tempnam', ['&s', '&s'], 's'],
        'sys_get_temp_dir' => ['sys_get_temp_dir', [], 's'],
        'glob' => ['glob', ['&s', 'i=0'], 'ls'],
        'scandir' => ['scandir', ['&s'], 'ls'],
        'readlink' => ['readlink', ['&s'], 'os'],
        'symlink' => ['symlink', ['&s', '&s'], 'b'],
        'file' => ['file_lines', ['&s', 'i=0'], 'ls'],
        'getenv' => ['getenv', ['&s'], 'os'],
        'putenv' => ['putenv', ['&s'], 'b'],
        'ini_set' => ['ini_set', ['&s', '&s'], 'os'],
        'ini_get' => ['ini_get', ['&s'], 'os'],
        'set_error_handler' => ['set_error_handler', ['&m'], 'u'],
        'restore_error_handler' => ['restore_error_handler', [], 'b'],
        'set_exception_handler' => ['set_exception_handler', ['&m'], 'u'],
        'error_reporting' => ['error_reporting', ['?i'], 'i'],
        'error_log' => ['error_log', ['&s'], 'b'],
        'trigger_error' => ['trigger_error', ['&s', 'i=1024'], 'Sb'],
        'gc_collect_cycles' => ['gc_collect_cycles', [], 'i'],
        'gc_disable' => ['gc_disable', [], 'u'],
        'gc_enable' => ['gc_enable', [], 'u'],
        'memory_get_usage' => ['memory_get_usage', ['b=false'], 'i'],
        'memory_get_peak_usage' => ['memory_get_peak_usage', ['b=false'], 'i'],
        'usleep' => ['usleep', ['i'], 'u'],
        'sleep' => ['sleep', ['i'], 'i'],
        'extension_loaded' => ['extension_loaded', ['&s'], 'b'],
        'function_exists' => ['function_exists', ['&s'], 'b'],
        'phpversion' => ['phpversion', ['?&s'], 'os'],
        'php_sapi_name' => ['php_sapi_name', [], 's'],
        'error_get_last' => ['error_get_last', [], 'om'],
        'clearstatcache' => ['clearstatcache', [], 'u'],
        'spl_autoload_register' => ['spl_autoload_register', ['?&m'], 'b'],
        'setlocale' => ['setlocale', ['i', '&s'], 'os'],
        'date' => ['date', ['&s', '?i'], 's'],
        'strtotime' => ['strtotime', ['&s'], 'oi'],
        'json_last_error' => ['json_last_error', [], 'i'],
        'json_last_error_msg' => ['json_last_error_msg', [], 's'],
        'preg_last_error' => ['preg_last_error', [], 'i'],
        'preg_last_error_msg' => ['preg_last_error_msg', [], 's'],
        'preg_quote' => ['preg_quote', ['&s', '?&s'], 's'],
        'token_name' => ['token_name', ['i'], 's'],
        'spl_object_id' => ['spl_object_id', ['&m'], 'i'],
        'spl_object_hash' => ['spl_object_hash', ['&m'], 's'],
        'class_exists' => ['class_exists', ['&s', 'b=true'], 'b'],
        'interface_exists' => ['interface_exists', ['&s', 'b=true'], 'b'],
        'trait_exists' => ['trait_exists', ['&s', 'b=true'], 'b'],
        'enum_exists' => ['enum_exists', ['&s', 'b=true'], 'b'],
        'get_class' => ['get_class', ['&m'], 's'],
        'get_parent_class' => ['get_parent_class_of', ['&m'], 'os'],
        'get_object_vars' => ['get_object_vars', ['&m'], 'mkm'],
        'method_exists' => ['method_exists', ['&m', '&s'], 'b'],
        'property_exists' => ['property_exists', ['&m', '&s'], 'b'],
        'is_callable' => ['is_callable', ['&m'], 'b'],
        'is_a' => ['is_a_name', ['&m', '&s', 'b=false'], 'b'],
        'is_subclass_of' => ['is_subclass_of_name', ['&m', '&s', 'b=true'], 'b'],
        'defined' => ['defined', ['&s'], 'b'],
        'constant' => ['constant', ['&s'], 'Sm'],
        'define' => ['define', ['&s', 'm'], 'b'],
        'fwrite' => ['fwrite', ['&r', '&s'], 'oi'],
        'fputs' => ['fwrite', ['&r', '&s'], 'oi'],
        'fclose' => ['fclose', ['&r'], 'b'],
        'fopen' => ['fopen', ['&s', '&s'], 'or'],
        'fflush' => ['fflush', ['&r'], 'b'],
        'flock' => ['flock', ['&r', 'i'], 'b'],
        'fgets' => ['fgets', ['&r'], 'os'],
        'fread' => ['fread', ['&r', 'i'], 'os'],
        'feof' => ['feof', ['&r'], 'b'],
        'ftruncate' => ['ftruncate', ['&r', 'i'], 'b'],
        'rewind' => ['rewind', ['&r'], 'b'],
        'stream_get_contents' => ['stream_get_contents', ['&r'], 'os'],
        'stream_set_blocking' => ['stream_set_blocking', ['&r', 'b'], 'b'],
        'is_resource' => ['is_resource', ['&m'], 'b'],
        'gzdeflate' => ['gzdeflate', ['&s'], 'os'],
        'gzinflate' => ['gzinflate', ['&s'], 'os'],
        'serialize' => ['serialize', ['&m'], 's'],
        'unserialize' => ['unserialize', ['&s'], 'm'],
        'igbinary_serialize' => ['serialize', ['&m'], 's'],
        'igbinary_unserialize' => ['unserialize', ['&s'], 'm'],
        'array_is_list' => ['array_is_list_val', ['&x'], 'b'],
        'debug_print_backtrace' => ['debug_print_backtrace', [], 'u'],
        'cli_set_process_title' => ['cli_set_process_title', ['&s'], 'b'],
        'getmypid' => ['getmypid', [], 'i'],
        'gethostname' => ['gethostname', [], 's'],
        'get_include_path' => ['get_include_path', [], 's'],
        'get_included_files' => ['get_included_files', [], 'ls'],
        'get_loaded_extensions' => ['get_loaded_extensions', [], 'ls'],
        'get_declared_classes' => ['get_declared_classes', [], 'ls'],
        'get_declared_interfaces' => ['get_declared_interfaces', [], 'ls'],
        'get_defined_constants' => ['get_defined_constants', ['b=false'], 'mkm'],
        'get_defined_functions' => ['get_defined_functions', [], 'mkm'],
        'opcache_get_status' => ['opcache_get_status', [], 'om'],
        'get_cfg_var' => ['get_cfg_var', ['&s'], 'os'],
        'filter_var' => ['filter_var', ['&m', 'i=516', 'm=Mixed::Null'], 'm'],
        'getopt' => ['getopt', ['&s', 'L=List::new()'], 'mkm'],
        'exec' => ['exec', ['&s'], 'os'],
        'passthru' => ['passthru', ['&s'], 'u'],
        'array_key_last_str' => ['array_key_last_str', ['&x'], 'os'],
        'nl_langinfo' => ['nl_langinfo_eol', ['i'], 's'],
        'str_word_count' => ['str_word_count', ['&s'], 'i'],
        'hex2bin' => ['hex2bin', ['&s'], 'os'],
        'strstr' => ['strstr', ['&s', '&s', 'b=false'], 'os'],
        'substr_compare' => ['substr_compare', ['&s', '&s', 'i', '?i', 'b=false'], 'i'],
        'pack' => ['pack', ['&s', '&m'], 's'],
        'lz4_compress' => ['lz4_compress', ['&s'], 'os'],
        'lz4_uncompress' => ['lz4_uncompress', ['&s'], 'os'],
        'parse_url' => ['parse_url', ['&s', 'i=-1'], 'm'],
        'checkdate' => ['checkdate', ['i', 'i', 'i'], 'b'],
        'array_key_exists_mixed' => ['array_key_exists_mixed', ['&k', '&m'], 'b'],
        'array_replace_recursive' => ['array_replace_recursive', ['&m', '&m'], 'm'],
        'strnatcmp' => ['strnatcmp', ['&s', '&s'], 'i'],
        'strnatcasecmp' => ['strnatcasecmp', ['&s', '&s'], 'i'],
        'posix_kill' => ['posix_kill', ['i', 'i'], 'b'],
        'posix_get_last_error' => ['posix_get_last_error', [], 'i'],
        'posix_strerror' => ['posix_strerror', ['i'], 's'],
        'readline' => ['readline', ['?&s'], 'os'],
        'libxml_use_internal_errors' => ['libxml_use_internal_errors', ['b=false'], 'b'],
        'libxml_clear_errors' => ['libxml_clear_errors', [], 'u'],
        'libxml_get_errors' => ['libxml_get_errors', [], 'L'],
        'strncmp_ci' => ['strncasecmp', ['&s', '&s', 'i'], 'i'],
        'debug_zval_refcount' => ['debug_zval_refcount', ['&m'], 'i'],
        'is_iterable' => ['is_iterable_val', ['&m'], 'b'],
        'is_countable' => ['is_countable_val', ['&m'], 'b'],
        'is_scalar' => ['is_scalar_val', ['&m'], 'b'],
        'settype' => ['settype', ['&m', '&s'], 'b'],
        'array_walk_recursive' => ['array_walk_recursive', ['&m', '&m'], 'b'],
        'fprintf' => ['fprintf', ['&r', '&s', 'V'], 'i'],
        'printf' => ['printf', ['&s', 'V'], 'i'],
        'sprintf' => ['sprintf', ['&s', 'V'], 'Ss'],
        'vsprintf' => ['vsprintf', ['&s', '&L'], 'Ss'],
        'str_ireplace' => ['str_ireplace', ['&s', '&s', '&s'], 's'],
    ];

    private function simple(BodyEmitter $b, Expr\FuncCall $call, array $args, array $spec): Val
    {
        [$fn, $params, $ret] = $spec;
        $codes = [];
        $positional = [];
        foreach ($args as $a) {
            $positional[] = $a;
        }
        foreach ($params as $i => $p) {
            $optional = false;
            $byref = false;
            $default = null;
            if ($p[0] === '?') {
                $optional = true;
                $p = substr($p, 1);
            }
            if ($p[0] === '&') {
                $byref = true;
                $p = substr($p, 1);
            }
            if (str_contains($p, '=')) {
                [$p, $default] = explode('=', $p, 2);
            }
            $letter = $p;
            if ($letter === 'V') {
                // variadic FmtArg list
                $parts = [];
                foreach (array_slice($positional, $i) as $a) {
                    $v = $b->expr($a->value);
                    $parts[] = 'FmtArg::from(' . $this->fmtArg($b, $v) . ')';
                }
                $codes[] = '&[' . implode(', ', $parts) . ']';
                break;
            }
            $arg = $positional[$i] ?? null;
            if ($arg === null) {
                if ($optional) {
                    $codes[] = 'None';
                    continue;
                }
                $codes[] = ($byref ? '&' : '') . $this->defaultCode($letter, $default);
                continue;
            }
            $t = $this->letterType($letter);
            if ($letter === 'x') {
                $v = $b->expr($arg->value);
                $codes[] = ($byref ? '&' : '') . $v->code;
                continue;
            }
            if ($letter === 'n') {
                $v = $b->expr($arg->value);
                $codes[] = $v->type->kind === RustType::FLOAT ? 'Num::Float(' . $v->code . ')' : 'Num::Int(' . $b->casts->convert($v->code, $v->type, RustType::int()) . ')';
                continue;
            }
            if ($letter === 'm') {
                // a Mixed argument takes the value with its declared type: no narrowing unwrap can fail
                $rv = $b->rawValue($arg->value);
                $code = $b->casts->convert($rv->code, $rv->type, RustType::mixed());
            } else {
                $code = $b->exprTo($arg->value, $t);
            }
            if ($optional) {
                $codes[] = 'Some(' . ($byref ? '&' : '') . $code . ')';
            } else {
                $codes[] = ($byref ? '&' : '') . $code;
            }
        }
        $code = $fn . '(' . implode(', ', $codes) . ')';
        if ($ret[0] === 'S') {
            $code .= '?';
            $ret = substr($ret, 1);
        }
        if ($ret === 'n') {
            return $b->narrow(new Val($code, RustType::rtGeneric('Num', [])), $call);
        }
        $rt = $this->retType($ret);
        if ($ret === 'u') {
            return new Val($code, RustType::unit());
        }
        return new Val($code, $rt);
    }

    private function fmtArg(BodyEmitter $b, Val $v): string
    {
        return match ($v->type->kind) {
            RustType::INT, RustType::FLOAT, RustType::STR, RustType::BOOL, RustType::ARRAY_KEY, RustType::MIXED => $v->code,
            RustType::OPTION => $v->code,
            default => $b->casts->convert($v->code, $v->type, RustType::mixed()),
        };
    }

    private function letterType(string $l): RustType
    {
        return match ($l) {
            's' => RustType::str(),
            'i' => RustType::int(),
            'f' => RustType::float(),
            'b' => RustType::bool(),
            'k' => RustType::arrayKey(),
            'L' => RustType::list(RustType::mixed()),
            'r' => RustType::resource(),
            default => RustType::mixed(),
        };
    }

    private function retType(string $r): RustType
    {
        if ($r[0] === 'o') {
            return RustType::option($this->retType(substr($r, 1)));
        }
        return match ($r) {
            'ls' => RustType::list(RustType::str()),
            'mss' => RustType::map(RustType::str(), RustType::str()),
            'mkm' => RustType::map(RustType::arrayKey(), RustType::mixed()),
            'L' => RustType::list(RustType::mixed()),
            'u' => RustType::unit(),
            default => $this->letterType($r),
        };
    }

    private function defaultCode(string $letter, ?string $default): string
    {
        if ($default !== null && $letter !== 's') {
            return $default;
        }
        return match ($letter) {
            's' => $default === null ? 'Str::empty()' : Names::strLit($default),
            'i' => '0',
            'f' => '0.0',
            'b' => 'false',
            'L' => 'List::new()',
            default => 'Mixed::Null',
        };
    }

    /** Emit `$a` as `&T` where T is the natural container type (List/Map). */
    private function container(BodyEmitter $b, Expr $e): Val
    {
        $v = $b->expr($e);
        $t = $v->type;
        if ($t->kind === RustType::OPTION) {
            $v = new Val($v->code . '.unwrap_or_default()', $t->inner());
            $t = $t->inner();
        }
        if ($t->kind === RustType::TUPLE) {
            $lt = RustType::list($b->types()->combine($t->params));
            return new Val($b->casts->convert($v->code, $t, $lt), $lt);
        }
        if ($t->kind === RustType::SHAPE) {
            $types = [];
            foreach ($t->fields as [$ft, $opt]) {
                $types[] = $ft;
            }
            $all_int = true;
            foreach ($t->fields as $k => $_) {
                if ((string) (int) $k !== (string) $k) {
                    $all_int = false;
                }
            }
            $mt = RustType::map($all_int ? RustType::int() : RustType::str(), $b->types()->combine($types));
            return new Val($b->casts->convert($v->code, $t, $mt), $mt);
        }
        if ($t->kind === RustType::MIXED) {
            $mt = RustType::map(RustType::arrayKey(), RustType::mixed());
            return new Val($b->casts->convert($v->code, $t, $mt), $mt);
        }
        if ($t->kind === RustType::UNION) {
            $mt = null;
            foreach ($t->params as $m) {
                if ($m->kind === RustType::MAP || $m->kind === RustType::LIST) {
                    $mt = $mt === null ? $m : RustType::map(RustType::arrayKey(), RustType::mixed());
                }
            }
            if ($mt === null) {
                $mt = RustType::map(RustType::arrayKey(), RustType::mixed());
            }
            return new Val($b->casts->convert($v->code, $t, $mt), $mt);
        }
        if ($t->kind === RustType::RT_GENERIC && $t->name === 'Generator') {
            $mt = RustType::map($t->params[0]->kind === RustType::INT ? RustType::int() : ($t->params[0]->kind === RustType::STR ? RustType::str() : RustType::arrayKey()), $t->params[1]);
            return new Val($b->casts->convert($v->code, $t, $mt), $mt);
        }
        return $v;
    }

    private function isList(RustType $t): bool
    {
        return $t->kind === RustType::LIST;
    }

    /** Emit a callable argument as a Rust closure with the given signature. */
    private function callback(BodyEmitter $b, Expr $e, array $params, RustType $ret): string
    {
        $target = RustType::closure($params, $ret);
        $v = $b->expr($e);
        if ($v->type->kind === RustType::STR) {
            // function name string, e.g. 'strtolower'
            $lit = $e instanceof \PhpParser\Node\Scalar\String_ ? $e->value : null;
            if ($lit !== null) {
                $args = [];
                $decls = [];
                foreach ($params as $i => $p) {
                    $decls[] = '__p' . $i . ': ' . $p->toRust();
                    $args[] = new Arg(new Expr\Variable('__cb' . $i));
                }
                $saved = $b->vars;
                foreach ($params as $i => $p) {
                    $b->vars['__cb' . $i] = $p;
                    $b->late['__cb' . $i] = false;
                }
                $call = new Expr\FuncCall(new \PhpParser\Node\Name($lit), $args);
                $inner = $this->emit($b, $call, strtolower(ltrim($lit, '\\')), $args);
                $b->vars = $saved;
                if ($inner !== null) {
                    $lets = [];
                    foreach ($params as $i => $p) {
                        $lets[] = 'let __cb' . $i . ' = __p' . $i . ';';
                    }
                    $code = $b->casts->convert($inner->code, $inner->type, $ret);
                    return 'Rc::new(move |' . implode(', ', $decls) . '| -> Result<' . $ret->toRust() . ', Throw> { ' . implode(' ', $lets) . ' Ok(' . $code . ') }) as ' . $target->toRust();
                }
            }
        }
        return $b->casts->convert($v->code, $v->type, $target);
    }

    /** Call a runtime fn expecting a closure `FnMut(args) -> Result<_, Throw>` given a PHP callable arg. */
    private function cb(BodyEmitter $b, Expr $e, array $params, RustType $ret): string
    {
        $c = $this->callback($b, $e, $params, $ret);
        $names = [];
        foreach ($params as $i => $_) {
            $names[] = '__a' . $i;
        }
        return '{ let __f = ' . $c . '; move |' . implode(', ', $names) . '| __f(' . implode(', ', $names) . ') }';
    }

    // ------------------------------------------------------------------ specific functions

    private function f_count(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $v = $b->expr($args[0]->value);
        $t = $v->type;
        if ($t->kind === RustType::LIST || $t->kind === RustType::MAP) {
            return new Val($v->code . '.count()', RustType::int());
        }
        if ($t->kind === RustType::TUPLE) {
            return new Val('{ let _ = ' . $v->code . '; ' . count($t->params) . 'i64 }', RustType::int());
        }
        if ($t->kind === RustType::SHAPE) {
            $c = $this->container($b, $args[0]->value);
            return new Val($c->code . '.count()', RustType::int());
        }
        if ($t->kind === RustType::CLASS_) {
            $cls = $b->program->classOf($t);
            $m = $cls !== null ? $b->program->findMethod($cls, 'count') : null;
            if ($m !== null) {
                return new Val($v->code . '.' . $m->rustName() . '()?', RustType::int());
            }
        }
        if ($t->kind === RustType::RT_GENERIC) {
            return new Val($v->code . '.count()', RustType::int());
        }
        return new Val('count(&' . $b->casts->convert($v->code, $t, RustType::mixed()) . ')', RustType::int());
    }

    private function f_sizeof(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->f_count($b, $call, $args);
    }

    private function f_is_string(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->typeCheck($b, $args[0]->value, 'is_string', [RustType::STR]);
    }

    private function f_is_int(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->typeCheck($b, $args[0]->value, 'is_int', [RustType::INT]);
    }

    private function f_is_integer(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->f_is_int($b, $call, $args);
    }

    private function f_is_float(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->typeCheck($b, $args[0]->value, 'is_float', [RustType::FLOAT]);
    }

    private function f_is_bool(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->typeCheck($b, $args[0]->value, 'is_bool', [RustType::BOOL]);
    }

    private function f_is_array(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->typeCheck($b, $args[0]->value, 'is_array', [RustType::LIST, RustType::MAP, RustType::TUPLE, RustType::SHAPE]);
    }

    private function f_is_null(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $v = $b->expr($args[0]->value);
        if ($v->type->kind === RustType::OPTION) {
            return new Val($v->code . '.is_none()', RustType::bool());
        }
        if ($v->type->kind === RustType::MIXED) {
            return new Val($v->code . '.is_null()', RustType::bool());
        }
        if ($v->type->kind === RustType::UNIT) {
            return new Val('{ let _ = ' . $v->code . '; true }', RustType::bool());
        }
        return new Val('{ let _ = ' . $v->code . '; false }', RustType::bool());
    }

    private function f_is_object(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->typeCheck($b, $args[0]->value, 'is_object', [RustType::CLASS_, RustType::ANY_OBJECT, RustType::CLOSURE, RustType::DYN_CALLABLE, RustType::RT_GENERIC]);
    }

    private function f_is_callable_val(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->typeCheck($b, $args[0]->value, 'is_callable', [RustType::CLOSURE, RustType::DYN_CALLABLE]);
    }

    /** Type predicates: statically decided when the Rust type is known, else on Mixed/unions. */
    private function typeCheck(BodyEmitter $b, Expr $e, string $pred, array $kinds): Val
    {
        $v = $b->rawValue($e);
        $t = $v->type;
        $inner = $t->kind === RustType::OPTION ? $t->inner() : $t;
        if ($t->kind === RustType::MIXED) {
            return new Val($v->code . '.' . $pred . '()', RustType::bool());
        }
        if ($t->kind === RustType::UNION || ($t->kind === RustType::OPTION && $inner->kind === RustType::UNION)) {
            $u = $inner;
            $arms = [];
            foreach ($u->params as $m) {
                if (in_array($m->kind, $kinds, true) || ($pred === 'is_bool' && $m->kind === RustType::RT_GENERIC && str_starts_with($m->name, '__unit_'))
                    || ($pred === 'is_int' && $m->kind === RustType::ARRAY_KEY) || ($pred === 'is_string' && $m->kind === RustType::ARRAY_KEY)
                ) {
                    $unit = $m->kind === RustType::RT_GENERIC && str_starts_with($m->name, '__unit_');
                    if ($m->kind === RustType::ARRAY_KEY) {
                        $arms[] = $u->mangle() . '::ArrayKey(__k) => __k.' . ($pred === 'is_int' ? 'is_int()' : 'is_str()');
                    } else {
                        $arms[] = $u->mangle() . '::' . $m->variantName() . ($unit ? '' : '(_)') . ' => true';
                    }
                }
            }
            if (in_array($pred, ['is_null', 'is_int', 'is_float', 'is_string', 'is_bool', 'is_array', 'is_object', 'is_scalar'], true)) {
                $arms[] = $u->mangle() . '::Other__(__m) => __m.' . $pred . '()';
            }
            $code = 'match ' . ($t->kind === RustType::OPTION ? $v->code . '.unwrap_or_default_marker()' : $v->code) . ' { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => false }';
            if ($t->kind === RustType::OPTION) {
                $code = 'match ' . $v->code . ' { Some(__u) => match __u { ' . implode(', ', $arms) . ($arms ? ', ' : '') . '_ => false }, None => false }';
            }
            return new Val('(' . $code . ')', RustType::bool());
        }
        if ($t->kind === RustType::ARRAY_KEY && ($pred === 'is_int' || $pred === 'is_string')) {
            return new Val($v->code . '.' . ($pred === 'is_int' ? 'is_int()' : 'is_str()'), RustType::bool());
        }
        if (($t->kind === RustType::CLASS_ || ($t->kind === RustType::OPTION && $inner->kind === RustType::CLASS_))
            && in_array($pred, ['is_null', 'is_int', 'is_float', 'is_string', 'is_bool', 'is_array', 'is_object', 'is_scalar'], true)
        ) {
            // a non-leaf class value may carry a non-object through its escape variant
            $c = $b->program->classOf($inner);
            if ($c !== null && !$c->isLeaf()) {
                $h = $inner->toRust();
                $static = in_array($inner->kind, $kinds, true) ? 'true' : 'false';
                if ($t->kind === RustType::OPTION) {
                    return new Val('(match &' . $v->code . ' { None => ' . ($pred === 'is_null' ? 'true' : 'false') . ', Some(' . $h . '::Other__(__m)) => __m.' . $pred . '(), Some(_) => ' . $static . ' })', RustType::bool());
                }
                return new Val('(match &' . $v->code . ' { ' . $h . '::Other__(__m) => __m.' . $pred . '(), _ => ' . $static . ' })', RustType::bool());
            }
        }
        if ($t->kind === RustType::OPTION) {
            $in = in_array($inner->kind, $kinds, true);
            return new Val('{ let _ = ' . $v->code . '; ' . ($in ? $v->code . '.is_some()' : 'false') . ' }', RustType::bool());
        }
        $res = in_array($t->kind, $kinds, true) ? 'true' : 'false';
        return new Val('{ let _ = ' . $v->code . '; ' . $res . ' }', RustType::bool());
    }

    private function f_is_numeric(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $v = $b->expr($args[0]->value);
        $t = $v->type;
        if ($t->kind === RustType::STR) {
            return new Val('is_numeric(&' . $v->code . ')', RustType::bool());
        }
        if ($t->kind === RustType::INT || $t->kind === RustType::FLOAT) {
            return new Val('{ let _ = ' . $v->code . '; true }', RustType::bool());
        }
        return new Val($b->casts->convert($v->code, $t, RustType::mixed()) . '.is_numeric()', RustType::bool());
    }

    private function f_is_scalar(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->typeCheck($b, $args[0]->value, 'is_scalar', [RustType::INT, RustType::FLOAT, RustType::STR, RustType::BOOL, RustType::ARRAY_KEY]);
    }

    private function f_is_iterable(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->typeCheck($b, $args[0]->value, 'is_iterable', [RustType::LIST, RustType::MAP, RustType::TUPLE, RustType::SHAPE, RustType::RT_GENERIC]);
    }

    private function f_is_callable(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $v = $b->expr($args[0]->value);
        if ($v->type->kind === RustType::CLOSURE || $v->type->kind === RustType::DYN_CALLABLE) {
            return new Val('{ let _ = ' . $v->code . '; true }', RustType::bool());
        }
        return new Val('is_callable(&' . $b->casts->convert($v->code, $v->type, RustType::mixed()) . ')', RustType::bool());
    }

    private function f_in_array(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $hay = $this->container($b, $args[1]->value);
        $strict = isset($args[2]) && $args[2]->value instanceof Expr\ConstFetch && strtolower($args[2]->value->name->toString()) === 'true';
        $ht = $hay->type;
        $elem = $ht->kind === RustType::LIST ? $ht->inner() : $ht->params[1];
        $needle = $b->expr($args[0]->value);
        $ct = $this->commonElem($b, $needle->type, $elem);
        if ($ct->toRust() !== $elem->toRust()) {
            $hay = new Val($b->casts->convert($hay->code, $ht, $ht->kind === RustType::LIST ? RustType::list($ct) : RustType::map($ht->params[0], $ct)), $ht->kind === RustType::LIST ? RustType::list($ct) : RustType::map($ht->params[0], $ct));
        }
        $n = $b->casts->convert($needle->code, $needle->type, $ct);
        $fn = ($strict ? 'in_array' : 'in_array_loose') . ($hay->type->kind === RustType::LIST ? '_l' : '_m');
        return new Val($fn . '(&' . $n . ', &' . $hay->code . ')', RustType::bool());
    }

    private function commonElem(BodyEmitter $b, RustType $needle, RustType $elem): RustType
    {
        if ($needle->toRust() === $elem->toRust()) {
            return $elem;
        }
        if ($elem->kind === RustType::UNION && $b->casts->pickMember($elem, $needle) !== null) {
            return $elem;
        }
        if ($elem->kind === RustType::OPTION && $elem->inner()->toRust() === $needle->toRust()) {
            return $elem;
        }
        if ($needle->kind === RustType::OPTION && $needle->inner()->toRust() === $elem->toRust()) {
            return $needle;
        }
        if ($elem->kind === RustType::CLASS_ && $needle->kind === RustType::CLASS_) {
            $ec = $b->program->classOf($elem);
            $nc = $b->program->classOf($needle);
            if ($ec !== null && $nc !== null && $nc->isSubclassOf($ec)) {
                return $elem;
            }
            if ($ec !== null && $nc !== null && $ec->isSubclassOf($nc)) {
                return $needle;
            }
        }
        if (($elem->kind === RustType::ARRAY_KEY && ($needle->kind === RustType::INT || $needle->kind === RustType::STR))
            || ($needle->kind === RustType::ARRAY_KEY && ($elem->kind === RustType::INT || $elem->kind === RustType::STR))
        ) {
            return RustType::arrayKey();
        }
        if ($elem->kind === RustType::MIXED || $needle->kind === RustType::MIXED) {
            return RustType::mixed();
        }
        if (($elem->kind === RustType::INT && $needle->kind === RustType::FLOAT) || ($elem->kind === RustType::FLOAT && $needle->kind === RustType::INT)) {
            return RustType::float();
        }
        return RustType::mixed();
    }

    private function f_array_search(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $hay = $this->container($b, $args[1]->value);
        $ht = $hay->type;
        $elem = $ht->kind === RustType::LIST ? $ht->inner() : $ht->params[1];
        $needle = $b->expr($args[0]->value);
        $ct = $this->commonElem($b, $needle->type, $elem);
        if ($ct->toRust() !== $elem->toRust()) {
            $nt = $ht->kind === RustType::LIST ? RustType::list($ct) : RustType::map($ht->params[0], $ct);
            $hay = new Val($b->casts->convert($hay->code, $ht, $nt), $nt);
        }
        $n = $b->casts->convert($needle->code, $needle->type, $ct);
        if ($hay->type->kind === RustType::LIST) {
            return $b->narrow(new Val('array_search_l(&' . $n . ', &' . $hay->code . ')', RustType::option(RustType::int())), $call);
        }
        return $b->narrow(new Val('array_search_m(&' . $n . ', &' . $hay->code . ')', RustType::option($hay->type->params[0])), $call);
    }

    private function f_array_keys(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $c = $this->container($b, $args[0]->value);
        if (isset($args[1])) {
            $elem = $c->type->kind === RustType::LIST ? $c->type->inner() : $c->type->params[1];
            $needle = $b->exprTo($args[1]->value, $elem);
            $m = $c->type->kind === RustType::LIST ? $b->casts->convert($c->code, $c->type, RustType::map(RustType::int(), $elem)) : $c->code;
            $kt = $c->type->kind === RustType::LIST ? RustType::int() : $c->type->params[0];
            return new Val('array_keys_search_m(&' . $m . ', &' . $needle . ')', RustType::list($kt));
        }
        if ($c->type->kind === RustType::LIST) {
            return new Val('array_keys_l(&' . $c->code . ')', RustType::list(RustType::int()));
        }
        return new Val('array_keys_m(&' . $c->code . ')', RustType::list($c->type->params[0]));
    }

    private function f_array_values(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $c = $this->container($b, $args[0]->value);
        if ($c->type->kind === RustType::LIST) {
            return new Val($c->code, $c->type);
        }
        return new Val('array_values_m(&' . $c->code . ')', RustType::list($c->type->params[1]));
    }

    private function f_array_merge(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $res = $b->inferredOrMixed($call);
        $vals = [];
        $all_list = true;
        foreach ($args as $a) {
            if ($a->unpack) {
                $b->warn('array_merge with unpacking', $call);
                $c = $this->container($b, $a->value);
                $mm = RustType::map(RustType::arrayKey(), RustType::map(RustType::arrayKey(), RustType::mixed()));
                $vals[] = new Val('array_merge_m(&' . $b->casts->convert($c->code, $c->type, $mm) . '.values().map(|__x| __x.clone()).collect::<Vec<_>>().iter().collect::<Vec<_>>())', RustType::map(RustType::arrayKey(), RustType::mixed()));
                continue;
            }
            $c = $this->container($b, $a->value);
            if ($c->type->kind !== RustType::LIST) {
                $all_list = false;
            }
            $vals[] = $c;
        }
        if ($all_list && ($res->kind === RustType::LIST || $res->kind === RustType::MIXED)) {
            $elem = $res->kind === RustType::LIST ? $res->inner() : $b->types()->combine(array_map(fn(Val $v) => $v->type->inner(), $vals));
            $parts = [];
            foreach ($vals as $v) {
                $parts[] = '&' . $b->casts->convert($v->code, $v->type, RustType::list($elem));
            }
            return new Val('array_merge_l(&[' . implode(', ', $parts) . '])', RustType::list($elem));
        }
        $target = $res->kind === RustType::MAP ? $res : null;
        if ($target === null) {
            $keys = [];
            $elems = [];
            foreach ($vals as $v) {
                if ($v->type->kind === RustType::LIST) {
                    $keys[] = RustType::int();
                    $elems[] = $v->type->inner();
                } else {
                    $keys[] = $v->type->params[0];
                    $elems[] = $v->type->params[1];
                }
            }
            $kt = $b->types()->combine($keys);
            if ($kt->kind !== RustType::INT && $kt->kind !== RustType::STR) {
                $kt = RustType::arrayKey();
            }
            $target = RustType::map($kt, $b->types()->combine($elems));
        }
        $parts = [];
        foreach ($vals as $v) {
            $parts[] = '&' . $b->casts->convert($v->code, $v->type, $target);
        }
        return new Val('array_merge_m(&[' . implode(', ', $parts) . '])', $target);
    }

    private function f_array_replace(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $res = $b->inferredOrMixed($call);
        $a = $this->container($b, $args[0]->value);
        $target = $res->kind === RustType::MAP ? $res : ($a->type->kind === RustType::MAP ? $a->type : RustType::map(RustType::int(), $a->type->inner()));
        $code = $b->casts->convert($a->code, $a->type, $target);
        for ($i = 1; $i < count($args); $i++) {
            $c = $this->container($b, $args[$i]->value);
            $code = 'array_replace_m(&' . $code . ', &' . $b->casts->convert($c->code, $c->type, $target) . ')';
        }
        return new Val($code, $target);
    }

    private function f_array_map(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $res = $b->inferredOrMixed($call);
        if (count($args) > 2) {
            $a = $this->container($b, $args[1]->value);
            $c = $this->container($b, $args[2]->value);
            $at = $a->type->kind === RustType::LIST ? $a->type : RustType::list($a->type->params[1]);
            $ct = $c->type->kind === RustType::LIST ? $c->type : RustType::list($c->type->params[1]);
            $ret = $res->kind === RustType::LIST ? $res->inner() : ($res->kind === RustType::MAP ? $res->params[1] : RustType::mixed());
            $cb = $this->cb($b, $args[0]->value, [$at->inner(), $ct->inner()], $ret);
            return new Val('array_map2_l(&' . $b->casts->convert($a->code, $a->type, $at) . ', &' . $b->casts->convert($c->code, $c->type, $ct) . ', ' . $cb . ')?', RustType::list($ret));
        }
        $a = $this->container($b, $args[1]->value);
        if ($b->isNullLiteral($args[0]->value)) {
            return $a;
        }
        $elem = $a->type->kind === RustType::LIST ? $a->type->inner() : $a->type->params[1];
        $ret = $res->kind === RustType::LIST ? $res->inner() : ($res->kind === RustType::MAP ? $res->params[1] : null);
        if ($ret === null) {
            $ct = $b->inferred($args[0]->value);
            $ret = $ct !== null && $ct->kind === RustType::CLOSURE ? $ct->ret : RustType::mixed();
        }
        $cb = $this->cb($b, $args[0]->value, [$elem], $ret);
        if ($a->type->kind === RustType::LIST) {
            return new Val('array_map_l(&' . $a->code . ', ' . $cb . ')?', RustType::list($ret));
        }
        return new Val('array_map_m(&' . $a->code . ', ' . $cb . ')?', RustType::map($a->type->params[0], $ret));
    }

    private function f_array_filter(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $a = $this->container($b, $args[0]->value);
        $is_list = $a->type->kind === RustType::LIST;
        $kt = $is_list ? RustType::int() : $a->type->params[0];
        $vt = $is_list ? $a->type->inner() : $a->type->params[1];
        $suffix = $is_list ? '_l' : '_m';
        $res_t = RustType::map($kt, $vt);
        if (!isset($args[1]) || $b->isNullLiteral($args[1]->value)) {
            return new Val('array_filter' . $suffix . '(&' . $a->code . ')', $res_t);
        }
        $mode = 0;
        if (isset($args[2])) {
            $mv = $b->expr($args[2]->value);
            if ($args[2]->value instanceof Expr\ConstFetch) {
                $mode = strtoupper($args[2]->value->name->getLast()) === 'ARRAY_FILTER_USE_KEY' ? 1 : 2;
            }
        }
        if ($mode === 1) {
            $cb = $this->cb($b, $args[1]->value, [$kt], RustType::bool());
            return new Val('array_filter_key' . $suffix . '(&' . $a->code . ', ' . $cb . ')?', $res_t);
        }
        if ($mode === 2) {
            $cb = $this->cb($b, $args[1]->value, [$vt, $kt], RustType::bool());
            return new Val('array_filter_both' . $suffix . '(&' . $a->code . ', ' . $cb . ')?', $res_t);
        }
        $cb = $this->cb($b, $args[1]->value, [$vt], RustType::bool());
        return new Val('array_filter_cb' . $suffix . '(&' . $a->code . ', ' . $cb . ')?', $res_t);
    }

    private function f_array_reduce(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $a = $this->container($b, $args[0]->value);
        $res = $b->inferredOrMixed($call);
        $vt = $a->type->kind === RustType::LIST ? $a->type->inner() : $a->type->params[1];
        $init = isset($args[2]) ? $b->expr($args[2]->value, $res) : new Val($b->casts->convert('()', RustType::unit(), $res), $res);
        $cb = $this->cb($b, $args[1]->value, [$res, $vt], $res);
        $fn = $a->type->kind === RustType::LIST ? 'array_reduce_l' : 'array_reduce_m';
        return new Val($fn . '(&' . $a->code . ', ' . $init->code . ', ' . $cb . ')?', $res);
    }

    private function f_array_walk(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $b->warn('array_walk', $call);
        return new Val('true', RustType::bool());
    }

    private function f_array_unique(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $a = $this->container($b, $args[0]->value);
        $is_list = $a->type->kind === RustType::LIST;
        $vt = $is_list ? $a->type->inner() : $a->type->params[1];
        $kt = $is_list ? RustType::int() : $a->type->params[0];
        if ($vt->kind === RustType::STR) {
            return new Val('array_unique_str' . ($is_list ? '_l' : '') . '(&' . $a->code . ')', RustType::map($kt, $vt));
        }
        return new Val('array_unique' . ($is_list ? '_l' : '_m') . '(&' . $a->code . ')', RustType::map($kt, $vt));
    }

    private function f_array_reverse(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $a = $this->container($b, $args[0]->value);
        $preserve = isset($args[1]) && $b->truthy($args[1]->value) === 'true';
        if ($a->type->kind === RustType::LIST && !$preserve) {
            return new Val('array_reverse_l(&' . $a->code . ')', $a->type);
        }
        if ($a->type->kind === RustType::LIST) {
            $mt = RustType::map(RustType::int(), $a->type->inner());
            return new Val('array_reverse_m(&' . $b->casts->convert($a->code, $a->type, $mt) . ', true)', $mt);
        }
        return new Val('array_reverse_m(&' . $a->code . ', ' . ($preserve ? 'true' : 'false') . ')', $a->type);
    }

    private function f_array_slice(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $a = $this->container($b, $args[0]->value);
        $offset = $b->exprTo($args[1]->value, RustType::int());
        $len = isset($args[2]) && !$b->isNullLiteral($args[2]->value) ? $b->exprTo($args[2]->value, RustType::option(RustType::int())) : 'None';
        $preserve = isset($args[3]) ? $b->exprTo($args[3]->value, RustType::bool()) : 'false';
        if ($a->type->kind === RustType::LIST) {
            if ($preserve !== 'false') {
                $mt = RustType::map(RustType::int(), $a->type->inner());
                return new Val('array_slice_m(&' . $b->casts->convert($a->code, $a->type, $mt) . ', ' . $offset . ', ' . $len . ', ' . $preserve . ')', $mt);
            }
            return new Val('array_slice_l(&' . $a->code . ', ' . $offset . ', ' . $len . ')', $a->type);
        }
        return new Val('array_slice_m(&' . $a->code . ', ' . $offset . ', ' . $len . ', ' . $preserve . ')', $a->type);
    }

    private function f_array_splice(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $place = $this->arrayPlace($b, $b->place($args[0]->value));
        $pt = $place->type;
        $offset = $b->exprTo($args[1]->value, RustType::int());
        $len = isset($args[2]) && !$b->isNullLiteral($args[2]->value) ? $b->exprTo($args[2]->value, RustType::option(RustType::int())) : 'None';
        $vt = $pt->kind === RustType::LIST ? $pt->inner() : ($pt->kind === RustType::MAP ? $pt->params[1] : RustType::mixed());
        $repl = 'Vec::new()';
        if (isset($args[3])) {
            $r = $b->expr($args[3]->value);
            if ($r->type->kind === RustType::OPTION) {
                $r = new Val($r->code . '.unwrap_or_default()', $r->type->inner());
            }
            if ($r->type->kind === RustType::LIST || $r->type->kind === RustType::MAP || $r->type->kind === RustType::TUPLE || $r->type->kind === RustType::SHAPE) {
                $repl = $b->casts->convert($r->code, $r->type, RustType::list($vt)) . '.into_vec()';
            } else {
                $repl = 'vec![' . $b->casts->convert($r->code, $r->type, $vt) . ']';
            }
        }
        $fn = $pt->kind === RustType::LIST ? 'array_splice_l' : 'array_splice_m';
        return new Val('{ let __off = ' . $offset . '; let __len = ' . $len . '; let __repl = ' . $repl . '; let __r = ' . $place->modifyValue(fn(string $p) => $fn . '(&mut ' . $p . ', __off, __len, __repl)') . '; __r }', RustType::list($vt));
    }

    private function f_array_pop(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $place = $this->arrayPlace($b, $b->place($args[0]->value));
        $pt = $place->type;
        $vt = $pt->kind === RustType::LIST ? $pt->inner() : ($pt->kind === RustType::MAP ? $pt->params[1] : RustType::mixed());
        if ($pt->kind !== RustType::LIST && $pt->kind !== RustType::MAP) {
            return $b->narrow(new Val('{ let __r = ' . $place->modifyValue(fn(string $p) => 'mixed_pop(&mut ' . $p . ')') . '; __r }', RustType::option(RustType::mixed())), $call);
        }
        return $b->narrow(new Val('{ let __r = ' . $place->modifyValue(fn(string $p) => $p . '.pop()') . '; __r }', RustType::option($vt)), $call);
    }

    /**
     * For a place typed as a union with a single array member (e.g. `list<string>|false`), a place
     * narrowed to that member: reads convert to the member type and writes wrap it back.
     */
    private function arrayPlace(BodyEmitter $b, Place $place): Place
    {
        $pt = $place->type;
        if ($pt->kind !== RustType::UNION) {
            return $place;
        }
        $member = null;
        foreach ($pt->params as $m) {
            if ($m->kind === RustType::LIST || $m->kind === RustType::MAP) {
                if ($member !== null) {
                    return $place;
                }
                $member = $m;
            }
        }
        if ($member === null) {
            return $place;
        }
        $casts = $b->casts;
        return new Place(
            $member,
            fn() => $casts->convert($place->read(), $pt, $member),
            fn(string $v) => $place->write($casts->convert($v, $member, $pt)),
        );
    }

    private function f_array_shift(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $place = $this->arrayPlace($b, $b->place($args[0]->value));
        $pt = $place->type;
        $vt = $pt->kind === RustType::LIST ? $pt->inner() : ($pt->kind === RustType::MAP ? $pt->params[1] : RustType::mixed());
        if ($pt->kind !== RustType::LIST && $pt->kind !== RustType::MAP) {
            return $b->narrow(new Val('{ let __r = ' . $place->modifyValue(fn(string $p) => 'mixed_shift(&mut ' . $p . ')') . '; __r }', RustType::option(RustType::mixed())), $call);
        }
        return $b->narrow(new Val('{ let __r = ' . $place->modifyValue(fn(string $p) => $p . '.shift()') . '; __r }', RustType::option($vt)), $call);
    }

    private function f_array_push(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $place = $this->arrayPlace($b, $b->place($args[0]->value));
        $pt = $place->type;
        $vt = $pt->kind === RustType::LIST ? $pt->inner() : ($pt->kind === RustType::MAP ? $pt->params[1] : RustType::mixed());
        $code = '';
        $is_arr = $pt->kind === RustType::LIST || $pt->kind === RustType::MAP;
        for ($i = 1; $i < count($args); $i++) {
            $code .= '{ let __v = ' . $b->exprTo($args[$i]->value, $vt) . '; ' . $place->modify(fn(string $p) => $is_arr ? $p . '.push(__v);' : 'mixed_set(&mut ' . $p . ', None, __v);') . ' } ';
        }
        return new Val('{ ' . $code . ($is_arr ? $place->read() . '.count()' : 'count(&' . $place->read() . ')') . ' }', RustType::int());
    }

    private function f_array_unshift(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $place = $this->arrayPlace($b, $b->place($args[0]->value));
        $pt = $place->type;
        $vt = $pt->kind === RustType::LIST ? $pt->inner() : ($pt->kind === RustType::MAP ? $pt->params[1] : RustType::mixed());
        $code = '';
        for ($i = count($args) - 1; $i >= 1; $i--) {
            $code .= '{ let __v = ' . $b->exprTo($args[$i]->value, $vt) . '; ' . $place->modify(fn(string $p) => $p . '.unshift(__v);') . ' } ';
        }
        return new Val('{ ' . $code . $place->read() . '.count() }', RustType::int());
    }

    private function f_array_key_exists(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $sv = $b->expr($args[1]->value);
        $st = $sv->type->kind === RustType::OPTION ? $sv->type->inner() : $sv->type;
        $key = $b->literalKey($args[0]->value);
        if ($st->kind === RustType::SHAPE && $key !== null) {
            if (!isset($st->fields[$key])) {
                return new Val('{ let _ = ' . $sv->code . '; false }', RustType::bool());
            }
            $acc = $sv->type->kind === RustType::OPTION ? $sv->code . '.map_or(false, |__s| __s.' . Names::field($key) . '.is_some())' : $sv->code . '.' . Names::field($key) . '.is_some()';
            return new Val($st->fields[$key][1] ? $acc : '{ let _ = ' . $sv->code . '; true }', RustType::bool());
        }
        $c = $this->container($b, $args[1]->value);
        if ($c->type->kind === RustType::LIST) {
            return new Val('array_key_exists_l(' . $b->exprTo($args[0]->value, RustType::int()) . ', &' . $c->code . ')', RustType::bool());
        }
        $kt = $c->type->params[0];
        return new Val($c->code . '.contains_key(&' . $b->keyExpr($args[0]->value, $kt) . ')', RustType::bool());
    }

    private function f_key_exists(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->f_array_key_exists($b, $call, $args);
    }

    private function f_array_key_first(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $c = $this->container($b, $args[0]->value);
        if ($c->type->kind === RustType::LIST) {
            return $b->narrow(new Val('array_key_first_l(&' . $c->code . ')', RustType::option(RustType::int())), $call);
        }
        return $b->narrow(new Val('array_key_first_m(&' . $c->code . ')', RustType::option($c->type->params[0])), $call);
    }

    private function f_array_key_last(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $c = $this->container($b, $args[0]->value);
        if ($c->type->kind === RustType::LIST) {
            return $b->narrow(new Val('array_key_last_l(&' . $c->code . ')', RustType::option(RustType::int())), $call);
        }
        return $b->narrow(new Val('array_key_last_m(&' . $c->code . ')', RustType::option($c->type->params[0])), $call);
    }

    /** `current($a)`, `key($a)`: reads at the internal array pointer */
    private function pointerRead(BodyEmitter $b, Expr\FuncCall $call, array $args, string $method): Val
    {
        $c = $this->container($b, $args[0]->value);
        if ($c->type->kind !== RustType::LIST && $c->type->kind !== RustType::MAP) {
            $b->warn($method . ' on ' . $c->type->toRust(), $call);
            return new Val('{ let _ = ' . $c->code . '; None::<Mixed> }', RustType::option(RustType::mixed()));
        }
        if ($method === 'ptr_key') {
            $kt = $c->type->kind === RustType::LIST ? RustType::int() : $c->type->params[0];
            $code = $c->type->kind === RustType::LIST ? $c->code . '.ptr_key()' : $c->code . '.ptr_key().cloned()';
            return $b->narrow(new Val($code, RustType::option($kt)), $call);
        }
        $vt = $c->type->kind === RustType::LIST ? $c->type->inner() : $c->type->params[1];
        return $b->narrow(new Val($c->code . '.' . $method . '().cloned()', RustType::option($vt)), $call);
    }

    /** `next($a)`, `prev($a)`, `reset($a)`, `end($a)`: move the internal array pointer of the place */
    private function pointerMove(BodyEmitter $b, Expr\FuncCall $call, array $args, string $method): Val
    {
        $place = $this->arrayPlace($b, $b->place($args[0]->value));
        $pt = $place->type;
        if ($pt->kind !== RustType::LIST && $pt->kind !== RustType::MAP) {
            $b->warn($method . ' on ' . $pt->toRust(), $call);
            return new Val('None::<Mixed>', RustType::option(RustType::mixed()));
        }
        $vt = $pt->kind === RustType::LIST ? $pt->inner() : $pt->params[1];
        return $b->narrow(new Val('{ let __r = ' . $place->modifyValue(fn(string $p) => $p . '.' . $method . '().cloned()') . '; __r }', RustType::option($vt)), $call);
    }

    private function f_reset(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->pointerMove($b, $call, $args, 'ptr_reset');
    }

    private function f_current(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->pointerRead($b, $call, $args, 'ptr_current');
    }

    private function f_end(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->pointerMove($b, $call, $args, 'ptr_end');
    }

    private function f_key(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->pointerRead($b, $call, $args, 'ptr_key');
    }

    private function f_next(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->pointerMove($b, $call, $args, 'ptr_next');
    }

    private function f_prev(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->pointerMove($b, $call, $args, 'ptr_prev');
    }

    private function f_implode(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        if (count($args) === 1) {
            $c = $this->container($b, $args[0]->value);
            $sep = 'Str::empty()';
        } else {
            $sep = $b->exprTo($args[0]->value, RustType::str());
            $c = $this->container($b, $args[1]->value);
        }
        if ($c->type->kind === RustType::LIST) {
            return new Val('implode(&' . $sep . ', &' . $c->code . ')', RustType::str());
        }
        return new Val('implode_m(&' . $sep . ', &' . $c->code . ')', RustType::str());
    }

    private function f_join(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->f_implode($b, $call, $args);
    }

    private function f_explode(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $sep = $b->exprTo($args[0]->value, RustType::str());
        $s = $b->exprTo($args[1]->value, RustType::str());
        $limit = isset($args[2]) ? $b->exprTo($args[2]->value, RustType::int()) : 'i64::MAX';
        return new Val('explode(&' . $sep . ', &' . $s . ', ' . $limit . ')?', RustType::list(RustType::str()));
    }

    private function f_str_replace(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $search = $b->expr($args[0]->value);
        $replace = $b->expr($args[1]->value);
        $subject = $b->expr($args[2]->value);
        $st = $subject->type;
        if ($st->kind === RustType::MIXED && !isset($args[3])) {
            // string or array subject decided at runtime
            return new Val('str_replace_m(&' . $b->casts->convert($search->code, $search->type, RustType::mixed()) . ', &' . $b->casts->convert($replace->code, $replace->type, RustType::mixed()) . ', &' . $subject->code . ')', RustType::mixed());
        }
        if ($st->kind !== RustType::STR) {
            // array subject: map over values
            $c = $this->container($b, $args[2]->value);
            $inner = new Expr\FuncCall($call->name, [$args[0], $args[1], new Arg(new Expr\Variable('__sr'))], $call->getAttributes());
            $b->vars['__sr'] = RustType::str();
            $b->late['__sr'] = false;
            $v = $this->f_str_replace($b, $inner, $inner->getArgs());
            if ($c->type->kind === RustType::LIST) {
                return new Val('array_map_l(&' . $b->casts->convert($c->code, $c->type, RustType::list(RustType::str())) . ', |__sr: Str| -> Result<Str, Throw> { Ok(' . $v->code . ') })?', RustType::list(RustType::str()));
            }
            return new Val('array_map_m(&' . $b->casts->convert($c->code, $c->type, RustType::map($c->type->params[0], RustType::str())) . ', |__sr: Str| -> Result<Str, Throw> { Ok(' . $v->code . ') })?', RustType::map($c->type->params[0], RustType::str()));
        }
        $s = $subject->code;
        if (isset($args[3])) {
            $place = $b->place($args[3]->value);
            return new Val('{ let mut __c: i64 = 0; let __r = str_replace_count(&' . $b->casts->convert($search->code, $search->type, RustType::str()) . ', &' . $b->casts->convert($replace->code, $replace->type, RustType::str()) . ', &' . $s . ', &mut __c); ' . $place->write($b->casts->convert('__c', RustType::int(), $place->type)) . ' __r }', RustType::str());
        }
        if ($search->type->kind === RustType::STR) {
            return new Val('str_replace(&' . $search->code . ', &' . $b->casts->convert($replace->code, $replace->type, RustType::str()) . ', &' . $s . ')', RustType::str());
        }
        $sl = $b->casts->convert($search->code, $search->type, RustType::list(RustType::str()));
        if ($replace->type->kind === RustType::STR) {
            return new Val('str_replace_arr_s(&' . $sl . ', &' . $replace->code . ', &' . $s . ')', RustType::str());
        }
        $rl = $b->casts->convert($replace->code, $replace->type, RustType::list(RustType::str()));
        return new Val('str_replace_arr(&' . $sl . ', &' . $rl . ', &' . $s . ')', RustType::str());
    }

    private function f_substr_replace(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $s = $b->exprTo($args[0]->value, RustType::str());
        $r = $b->exprTo($args[1]->value, RustType::str());
        $o = $b->exprTo($args[2]->value, RustType::int());
        $l = isset($args[3]) ? $b->exprTo($args[3]->value, RustType::option(RustType::int())) : 'None';
        return new Val('substr_replace(&' . $s . ', &' . $r . ', ' . $o . ', ' . $l . ')', RustType::str());
    }

    private function f_strtr(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $s = $b->exprTo($args[0]->value, RustType::str());
        if (count($args) === 3) {
            return new Val('strtr(&' . $s . ', &' . $b->exprTo($args[1]->value, RustType::str()) . ', &' . $b->exprTo($args[2]->value, RustType::str()) . ')', RustType::str());
        }
        $pairs = $b->exprTo($args[1]->value, RustType::map(RustType::str(), RustType::str()));
        return new Val('strtr_pairs(&' . $s . ', &' . $pairs . ')', RustType::str());
    }

    private function f_max(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->minmax($b, $call, $args, 'max');
    }

    private function f_min(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->minmax($b, $call, $args, 'min');
    }

    private function minmax(BodyEmitter $b, Expr\FuncCall $call, array $args, string $fn): Val
    {
        $res = $b->inferredOrMixed($call);
        if (count($args) === 1) {
            $c = $this->container($b, $args[0]->value);
            $vt = $c->type->kind === RustType::LIST ? $c->type->inner() : $c->type->params[1];
            return new Val($fn . ($c->type->kind === RustType::LIST ? '_l' : '_m') . '(&' . $c->code . ')?', $vt);
        }
        $vals = array_map(fn(Arg $a) => $b->expr($a->value), $args);
        $t = $res->kind !== RustType::MIXED && $res->kind !== RustType::UNION ? $res : $vals[0]->type;
        foreach ($vals as $v) {
            $t = $b->casts->pickMember($t, $v->type) !== null ? $t : ($t->toRust() === $v->type->toRust() ? $t : (($t->kind === RustType::INT && $v->type->kind === RustType::FLOAT) || ($t->kind === RustType::FLOAT && $v->type->kind === RustType::INT) ? RustType::float() : $t));
        }
        $code = $b->casts->convert($vals[0]->code, $vals[0]->type, $t);
        for ($i = 1; $i < count($vals); $i++) {
            $code = $fn . '2(' . $code . ', ' . $b->casts->convert($vals[$i]->code, $vals[$i]->type, $t) . ')';
        }
        return new Val($code, $t);
    }

    private function f_array_sum(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $c = $this->container($b, $args[0]->value);
        $res = $b->inferredOrMixed($call);
        $vt = $c->type->kind === RustType::LIST ? $c->type->inner() : $c->type->params[1];
        if ($res->kind === RustType::FLOAT || $vt->kind === RustType::FLOAT) {
            $l = $b->casts->convert($c->code, $c->type, RustType::list(RustType::float()));
            return new Val('array_sum_f(&' . $l . ')', RustType::float());
        }
        $l = $b->casts->convert($c->code, $c->type, RustType::list(RustType::int()));
        return new Val('array_sum_i(&' . $l . ')', RustType::int());
    }

    private function f_array_product(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $c = $this->container($b, $args[0]->value);
        $l = $b->casts->convert($c->code, $c->type, RustType::list(RustType::int()));
        return new Val('array_product_i(&' . $l . ')', RustType::int());
    }

    private function f_array_flip(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $c = $this->container($b, $args[0]->value);
        if ($c->type->kind === RustType::LIST) {
            $l = $b->casts->convert($c->code, $c->type, RustType::list(RustType::str()));
            return new Val('array_flip_l(&' . $l . ')', RustType::map(RustType::str(), RustType::int()));
        }
        [$kt, $vt] = $c->type->params;
        $nk = $vt->kind === RustType::INT ? RustType::int() : ($vt->kind === RustType::STR ? RustType::str() : RustType::arrayKey());
        $m = $b->casts->convert($c->code, $c->type, RustType::map($kt, $nk));
        return new Val('array_flip_m(&' . $m . ')', RustType::map($nk, $kt));
    }

    private function f_array_fill(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $res = $b->inferredOrMixed($call);
        $start = $b->exprTo($args[0]->value, RustType::int());
        $n = $b->exprTo($args[1]->value, RustType::int());
        $v = $b->expr($args[2]->value);
        if ($start === '0i64' && $res->kind === RustType::LIST) {
            return new Val('array_fill_l(' . $n . ', ' . $b->casts->convert($v->code, $v->type, $res->inner()) . ')', $res);
        }
        return new Val('array_fill(' . $start . ', ' . $n . ', ' . $v->code . ')', RustType::map(RustType::int(), $v->type));
    }

    private function f_array_fill_keys(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $keys = $this->container($b, $args[0]->value);
        $v = $b->expr($args[1]->value);
        $kt = $keys->type->kind === RustType::LIST ? $keys->type->inner() : $keys->type->params[1];
        $kk = $kt->kind === RustType::INT ? RustType::int() : ($kt->kind === RustType::STR ? RustType::str() : RustType::arrayKey());
        $res = $b->inferredOrMixed($call);
        $vt = $res->kind === RustType::MAP ? $res->params[1] : $v->type;
        $vc = $b->casts->convert($v->code, $v->type, $vt);
        if ($keys->type->kind === RustType::LIST) {
            return new Val('array_fill_keys(&' . $b->casts->convert($keys->code, $keys->type, RustType::list($kk)) . ', ' . $vc . ')', RustType::map($kk, $vt));
        }
        return new Val('array_fill_keys_m(&' . $b->casts->convert($keys->code, $keys->type, RustType::map($keys->type->params[0], $kk)) . ', ' . $vc . ')', RustType::map($kk, $vt));
    }

    private function f_array_combine(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $keys = $this->container($b, $args[0]->value);
        $vals = $this->container($b, $args[1]->value);
        $kt = $keys->type->kind === RustType::LIST ? $keys->type->inner() : $keys->type->params[1];
        $kk = $kt->kind === RustType::INT ? RustType::int() : ($kt->kind === RustType::STR ? RustType::str() : RustType::arrayKey());
        $vt = $vals->type->kind === RustType::LIST ? $vals->type->inner() : $vals->type->params[1];
        return new Val('array_combine(&' . $b->casts->convert($keys->code, $keys->type, RustType::list($kk)) . ', &' . $b->casts->convert($vals->code, $vals->type, RustType::list($vt)) . ')?', RustType::map($kk, $vt));
    }

    private function f_array_diff(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $a = $this->container($b, $args[0]->value);
        $is_list = $a->type->kind === RustType::LIST;
        $vt = $is_list ? $a->type->inner() : $a->type->params[1];
        $kt = $is_list ? RustType::int() : $a->type->params[0];
        $code = $a->code;
        $cur_t = $a->type;
        for ($i = 1; $i < count($args); $i++) {
            $c = $this->container($b, $args[$i]->value);
            if ($is_list && $vt->kind === RustType::STR && $c->type->kind === RustType::LIST) {
                $code = 'array_diff_str_l(&' . $b->casts->convert($code, $cur_t, RustType::list(RustType::str())) . ', &' . $b->casts->convert($c->code, $c->type, RustType::list(RustType::str())) . ')';
                $cur_t = RustType::map(RustType::int(), RustType::str());
                $is_list = false;
                continue;
            }
            $ct = $c->type->kind === RustType::LIST ? RustType::list($vt) : RustType::map($c->type->params[0], $vt);
            $cc = $b->casts->convert($c->code, $c->type, $ct);
            if ($cur_t->kind === RustType::LIST && $ct->kind === RustType::LIST) {
                $code = 'array_diff_l(&' . $code . ', &' . $cc . ')';
            } else {
                $mt = $cur_t->kind === RustType::LIST ? RustType::map(RustType::int(), $vt) : $cur_t;
                $cm = $ct->kind === RustType::LIST ? $b->casts->convert($cc, $ct, RustType::map(RustType::int(), $vt)) : $cc;
                $code = 'array_diff_m(&' . $b->casts->convert($code, $cur_t, $mt) . ', &' . $cm . ')';
            }
            $cur_t = RustType::map($kt, $vt);
        }
        return new Val($code, RustType::map($kt, $vt));
    }

    private function f_array_diff_key(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $a = $this->container($b, $args[0]->value);
        $code = $a->code;
        $t = $a->type;
        for ($i = 1; $i < count($args); $i++) {
            $c = $this->container($b, $args[$i]->value);
            if ($t->kind === RustType::LIST) {
                $cm = $b->casts->convert($c->code, $c->type, RustType::map(RustType::int(), $c->type->kind === RustType::LIST ? $c->type->inner() : $c->type->params[1]));
                $code = 'array_diff_key_l(&' . $code . ', &' . $cm . ')';
                $t = RustType::map(RustType::int(), $t->inner());
            } else {
                $kt = $t->params[0];
                $cvt = $c->type->kind === RustType::LIST ? $c->type->inner() : $c->type->params[1];
                $cm = $b->casts->convert($c->code, $c->type, RustType::map($kt, $cvt));
                $code = 'array_diff_key(&' . $code . ', &' . $cm . ')';
            }
        }
        return new Val($code, $t);
    }

    private function f_array_intersect_key(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $a = $this->container($b, $args[0]->value);
        $t = $a->type->kind === RustType::LIST ? RustType::map(RustType::int(), $a->type->inner()) : $a->type;
        $code = $b->casts->convert($a->code, $a->type, $t);
        for ($i = 1; $i < count($args); $i++) {
            $c = $this->container($b, $args[$i]->value);
            $cvt = $c->type->kind === RustType::LIST ? $c->type->inner() : $c->type->params[1];
            $cm = $b->casts->convert($c->code, $c->type, RustType::map($t->params[0], $cvt));
            $code = 'array_intersect_key(&' . $code . ', &' . $cm . ')';
        }
        return new Val($code, $t);
    }

    private function f_array_intersect(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $a = $this->container($b, $args[0]->value);
        $c = $this->container($b, $args[1]->value);
        $vt = $a->type->kind === RustType::LIST ? $a->type->inner() : $a->type->params[1];
        $ct = $c->type->kind === RustType::LIST ? RustType::list($vt) : RustType::map($c->type->params[0], $vt);
        $cc = $b->casts->convert($c->code, $c->type, $ct);
        if ($a->type->kind === RustType::LIST && $ct->kind === RustType::LIST) {
            return new Val('array_intersect_l(&' . $a->code . ', &' . $cc . ')', RustType::map(RustType::int(), $vt));
        }
        $mt = $a->type->kind === RustType::LIST ? RustType::map(RustType::int(), $vt) : $a->type;
        $cm = $ct->kind === RustType::LIST ? $b->casts->convert($cc, $ct, RustType::map(RustType::int(), $vt)) : $cc;
        return new Val('array_intersect_m(&' . $b->casts->convert($a->code, $a->type, $mt) . ', &' . $cm . ')', $mt);
    }

    private function f_array_column(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $c = $this->container($b, $args[0]->value);
        $res = $b->inferredOrMixed($call);
        $elem = $res->kind === RustType::LIST ? $res->inner() : RustType::mixed();
        $vt = $c->type->kind === RustType::LIST ? $c->type->inner() : $c->type->params[1];
        $m = $b->casts->convert($c->code, $c->type, $c->type->kind === RustType::LIST ? RustType::map(RustType::int(), $vt) : $c->type);
        $key = $b->literalKey($args[1]->value);
        if ($vt->kind === RustType::SHAPE && $key !== null && isset($vt->fields[$key])) {
            [$ft, $opt] = $vt->fields[$key];
            $access = $opt ? '__v.' . Names::field($key) . '.clone()' : 'Some(__v.' . Names::field($key) . '.clone())';
            return new Val('array_column(&' . $m . ', |__v| ' . $access . ')', RustType::list($ft));
        }
        if ($vt->kind === RustType::MAP) {
            $kt = $vt->params[0];
            $k = $b->keyExpr($args[1]->value, $kt);
            return new Val('array_column(&' . $m . ', |__v| __v.get(&' . $k . ').cloned())', RustType::list($vt->params[1]));
        }
        if ($vt->kind === RustType::CLASS_ && $key !== null) {
            $cls = $b->program->classOf($vt);
            $f = $cls?->fields[$key] ?? null;
            if ($f !== null) {
                return new Val('array_column(&' . $m . ', |__v| Some(__v.' . $f->acc() . '_get()))', RustType::list($f->type));
            }
        }
        $k = $b->keyExpr($args[1]->value, RustType::arrayKey());
        return new Val('array_column(&' . $b->casts->convert($m, $c->type->kind === RustType::LIST ? RustType::map(RustType::int(), $vt) : $c->type, RustType::map(RustType::arrayKey(), RustType::mixed())) . ', |__v| mixed_get(__v, &' . $k . '))', RustType::list(RustType::mixed()));
    }

    private function f_array_chunk(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $c = $this->container($b, $args[0]->value);
        $vt = $c->type->kind === RustType::LIST ? $c->type->inner() : $c->type->params[1];
        $l = $b->casts->convert($c->code, $c->type, RustType::list($vt));
        return new Val('array_chunk_l(&' . $l . ', ' . $b->exprTo($args[1]->value, RustType::int()) . ')', RustType::list(RustType::list($vt)));
    }

    private function f_array_pad(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $c = $this->container($b, $args[0]->value);
        $vt = $c->type->kind === RustType::LIST ? $c->type->inner() : $c->type->params[1];
        $l = $b->casts->convert($c->code, $c->type, RustType::list($vt));
        return new Val('array_pad_l(&' . $l . ', ' . $b->exprTo($args[1]->value, RustType::int()) . ', ' . $b->exprTo($args[2]->value, $vt) . ')', RustType::list($vt));
    }

    private function f_range(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $a = $b->expr($args[0]->value);
        $c = $b->expr($args[1]->value);
        if ($a->type->kind === RustType::STR && $c->type->kind === RustType::STR) {
            return new Val('range_c(' . $a->code . '.as_bytes()[0], ' . $c->code . '.as_bytes()[0])', RustType::list(RustType::str()));
        }
        $step = isset($args[2]) ? $b->exprTo($args[2]->value, RustType::int()) : '1';
        return new Val('range_i(' . $b->casts->convert($a->code, $a->type, RustType::int()) . ', ' . $b->casts->convert($c->code, $c->type, RustType::int()) . ', ' . $step . ')', RustType::list(RustType::int()));
    }

    private function f_array_any(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->anyAll($b, $call, $args, 'array_any');
    }

    private function f_array_all(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->anyAll($b, $call, $args, 'array_all');
    }

    private function anyAll(BodyEmitter $b, Expr\FuncCall $call, array $args, string $fn): Val
    {
        $c = $this->container($b, $args[0]->value);
        $is_list = $c->type->kind === RustType::LIST;
        $kt = $is_list ? RustType::int() : $c->type->params[0];
        $vt = $is_list ? $c->type->inner() : $c->type->params[1];
        $cb = $this->cbFlexible($b, $args[1]->value, [$vt, $kt], RustType::bool());
        return new Val($fn . ($is_list ? '_l' : '_m') . '(&' . $c->code . ', ' . $cb . ')?', RustType::bool());
    }

    /** Callback that may declare fewer params than provided (extra args dropped). */
    private function cbFlexible(BodyEmitter $b, Expr $e, array $params, RustType $ret): string
    {
        $ct = $b->inferred($e);
        $n = $ct !== null && $ct->kind === RustType::CLOSURE ? count($ct->params) : count($params);
        $use = array_slice($params, 0, max(1, $n));
        $c = $this->callback($b, $e, $use, $ret);
        $names = [];
        foreach ($params as $i => $_) {
            $names[] = '__a' . $i;
        }
        $pass = array_slice($names, 0, count($use));
        return '{ let __f = ' . $c . '; move |' . implode(', ', $names) . '| { ' . implode(' ', array_map(fn($x) => 'let _ = &' . $x . ';', $names)) . ' __f(' . implode(', ', $pass) . ') } }';
    }

    private function f_array_find(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $c = $this->container($b, $args[0]->value);
        $is_list = $c->type->kind === RustType::LIST;
        $kt = $is_list ? RustType::int() : $c->type->params[0];
        $vt = $is_list ? $c->type->inner() : $c->type->params[1];
        $cb = $this->cbFlexible($b, $args[1]->value, [$vt, $kt], RustType::bool());
        return $b->narrow(new Val('array_find' . ($is_list ? '_l' : '_m') . '(&' . $c->code . ', ' . $cb . ')?', RustType::option($vt)), $call);
    }

    private function f_array_find_key(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $c = $this->container($b, $args[0]->value);
        $is_list = $c->type->kind === RustType::LIST;
        $kt = $is_list ? RustType::int() : $c->type->params[0];
        $vt = $is_list ? $c->type->inner() : $c->type->params[1];
        $m = $is_list ? $b->casts->convert($c->code, $c->type, RustType::map(RustType::int(), $vt)) : $c->code;
        $cb = $this->cbFlexible($b, $args[1]->value, [$vt, $kt], RustType::bool());
        return $b->narrow(new Val('array_find_key_m(&' . $m . ', ' . $cb . ')?', RustType::option($kt)), $call);
    }

    private function f_array_change_key_case(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $c = $this->container($b, $args[0]->value);
        $vt = $c->type->kind === RustType::LIST ? $c->type->inner() : $c->type->params[1];
        $m = $b->casts->convert($c->code, $c->type, RustType::map(RustType::str(), $vt));
        return new Val('array_change_key_case_lower(&' . $m . ')', RustType::map(RustType::str(), $vt));
    }

    private function f_array_count_values(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $c = $this->container($b, $args[0]->value);
        $vt = $c->type->kind === RustType::LIST ? $c->type->inner() : $c->type->params[1];
        $kk = $vt->kind === RustType::INT ? RustType::int() : ($vt->kind === RustType::STR ? RustType::str() : RustType::arrayKey());
        if ($c->type->kind === RustType::LIST) {
            return new Val('array_count_values_l(&' . $b->casts->convert($c->code, $c->type, RustType::list($kk)) . ')', RustType::map($kk, RustType::int()));
        }
        return new Val('array_count_values(&' . $b->casts->convert($c->code, $c->type, RustType::map($c->type->params[0], $kk)) . ')', RustType::map($kk, RustType::int()));
    }

    private function f_array_replace_recursive(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $a = $b->exprTo($args[0]->value, RustType::mixed());
        $c = $b->exprTo($args[1]->value, RustType::mixed());
        return $b->narrow(new Val('array_replace_recursive(&' . $a . ', &' . $c . ')', RustType::mixed()), $call);
    }

    private function f_array_multisort(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $place = $this->arrayPlace($b, $b->place($args[0]->value));
        return new Val('{ ' . $place->wrap('array_multisort_l(&mut ' . $place->mut() . ');') . ' true }', RustType::bool());
    }

    private function f_compact(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $parts = [];
        foreach ($args as $a) {
            $name = $b->literalKey($a->value);
            if ($name === null) {
                continue;
            }
            $v = $b->readVar($name);
            $parts[] = '__m.insert(' . Names::strLit($name) . ', ' . $b->casts->convert($v->code, $v->type, RustType::mixed()) . ');';
        }
        return new Val('{ let mut __m: Map<Str, Mixed> = Map::new(); ' . implode(' ', $parts) . ' __m }', RustType::map(RustType::str(), RustType::mixed()));
    }

    // sorting

    private function sortInPlace(BodyEmitter $b, Expr\FuncCall $call, array $args, string $fn, bool $has_cb, bool $renumbers): Val
    {
        $place = $this->arrayPlace($b, $b->place($args[0]->value));
        $pt = $place->type;
        $is_list = $pt->kind === RustType::LIST;
        if (!$is_list && $pt->kind !== RustType::MAP) {
            $b->warn($fn . ' on ' . $pt->toRust(), $call);
            return new Val('true', RustType::bool());
        }
        $vt = $is_list ? $pt->inner() : $pt->params[1];
        $kt = $is_list ? RustType::int() : $pt->params[0];
        $cb = '';
        $pre = '';
        if ($has_cb) {
            $params = $fn === 'uksort' ? [$kt, $kt] : [$vt, $vt];
            // the callback may capture the container: build it before the container is borrowed mutably
            $pre = 'let __cb = ' . $this->cb($b, $args[1]->value, $params, RustType::int()) . '; ';
            $cb = ', __cb';
        }
        $flag_string = isset($args[1]) && !$has_cb && $args[1]->value instanceof Expr\ConstFetch && strtoupper($args[1]->value->name->getLast()) === 'SORT_STRING';
        if ($renumbers) {
            // sort/rsort/usort: result is a list
            if ($is_list) {
                $rf = $flag_string ? 'sort_flag_string_l' : $fn . '_l';
                $stmt = $place->wrap($rf . '(&mut ' . $place->mut() . $cb . ')' . ($has_cb ? '?' : '') . ';');
                return new Val('{ ' . $pre . $stmt . ' true }', RustType::bool());
            }
            $tmp = '__sorted';
            $stmt = 'let ' . $tmp . ' = ' . $fn . '_m(&' . $place->read() . $cb . ')' . ($has_cb ? '?' : '') . '; ' . $place->write($b->casts->convert($tmp, RustType::list($vt), $pt));
            return new Val('{ ' . $pre . $stmt . ' true }', RustType::bool());
        }
        // key-preserving sorts on lists become maps
        if ($is_list) {
            $mt = RustType::map(RustType::int(), $vt);
            $stmt = 'let mut __m = ' . $b->casts->convert($place->read(), $pt, $mt) . '; ' . ($flag_string ? 'ksort_flag_string' : $fn . '_m') . '(&mut __m' . $cb . ')' . ($has_cb ? '?' : '') . '; ' . $place->write($b->casts->convert('__m', $mt, $pt));
            return new Val('{ ' . $pre . $stmt . ' true }', RustType::bool());
        }
        $rf = $flag_string && $fn === 'ksort' ? 'ksort_flag_string' : $fn . '_m';
        $stmt = $place->wrap($rf . '(&mut ' . $place->mut() . $cb . ')' . ($has_cb ? '?' : '') . ';');
        return new Val('{ ' . $pre . $stmt . ' true }', RustType::bool());
    }

    private function f_sort(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->sortInPlace($b, $call, $args, 'sort', false, true);
    }

    private function f_rsort(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->sortInPlace($b, $call, $args, 'rsort', false, true);
    }

    private function f_usort(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->sortInPlace($b, $call, $args, 'usort', true, true);
    }

    private function f_uasort(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->sortInPlace($b, $call, $args, 'uasort', true, false);
    }

    private function f_uksort(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->sortInPlace($b, $call, $args, 'uksort', true, false);
    }

    private function f_ksort(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->sortInPlace($b, $call, $args, 'ksort', false, false);
    }

    private function f_krsort(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->sortInPlace($b, $call, $args, 'krsort', false, false);
    }

    private function f_asort(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->sortInPlace($b, $call, $args, 'asort', false, false);
    }

    private function f_arsort(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->sortInPlace($b, $call, $args, 'arsort', false, false);
    }

    private function f_shuffle(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $place = $this->arrayPlace($b, $b->place($args[0]->value));
        return new Val('{ ' . $place->wrap('shuffle_l(&mut ' . $place->mut() . ');') . ' true }', RustType::bool());
    }

    // preg

    private function f_preg_match(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $pat = $b->exprTo($args[0]->value, RustType::str());
        $s = $b->exprTo($args[1]->value, RustType::str());
        $flags = isset($args[3]) ? $b->exprTo($args[3]->value, RustType::int()) : '0';
        $offset = isset($args[4]) ? $b->exprTo($args[4]->value, RustType::int()) : '0';
        if (isset($args[2])) {
            $place = $b->place($args[2]->value);
            $pt = $place->type;
            $inner = $pt->kind === RustType::OPTION ? $pt->inner() : $pt;
            $tmp = '__m';
            $mt = RustType::map(RustType::arrayKey(), $flags === '0' ? RustType::str() : RustType::mixed());
            $fn = $flags === '0' ? 'preg_match_groups' : 'preg_match_groups_flags';
            $flag_arg = $flags === '0' ? '' : ', ' . $flags;
            $store = $b->casts->convert($tmp, $mt, $pt);
            return new Val('{ let (__r, ' . $tmp . ') = ' . $fn . '(&' . $pat . ', &' . $s . $flag_arg . ', ' . $offset . ')?; ' . $place->write($store) . ' __r }', RustType::int());
        }
        return new Val('preg_match(&' . $pat . ', &' . $s . ', ' . $offset . ')?', RustType::int());
    }

    private function f_preg_match_all(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $pat = $b->exprTo($args[0]->value, RustType::str());
        $s = $b->exprTo($args[1]->value, RustType::str());
        $flags = isset($args[3]) ? $b->exprTo($args[3]->value, RustType::int()) : '1';
        if (isset($args[2])) {
            $place = $b->place($args[2]->value);
            $mt = RustType::map(RustType::arrayKey(), RustType::mixed());
            return new Val('{ let (__r, __m) = preg_match_all(&' . $pat . ', &' . $s . ', ' . $flags . ')?; ' . $place->write($b->casts->convert('__m', $mt, $place->type)) . ' __r }', RustType::int());
        }
        return new Val('preg_match_all(&' . $pat . ', &' . $s . ', ' . $flags . ')?.0', RustType::int());
    }

    private function f_preg_replace(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $pat = $b->expr($args[0]->value);
        $rep = $b->expr($args[1]->value);
        $subj = $b->expr($args[2]->value);
        $limit = isset($args[3]) ? $b->exprTo($args[3]->value, RustType::int()) : '-1';
        if ($subj->type->kind !== RustType::STR && $subj->type->kind !== RustType::OPTION) {
            $c = $this->container($b, $args[2]->value);
            $vt = RustType::str();
            $lst = $b->casts->convert($c->code, $c->type, $c->type->kind === RustType::LIST ? RustType::list($vt) : RustType::map($c->type->params[0], $vt));
            $p = $b->casts->convert($pat->code, $pat->type, RustType::str());
            $r = $b->casts->convert($rep->code, $rep->type, RustType::str());
            $fn = $c->type->kind === RustType::LIST ? 'array_map_l' : 'array_map_m';
            return new Val($fn . '(&' . $lst . ', |__s: Str| preg_replace(&' . $p . ', &' . $r . ', &__s, ' . $limit . '))?', $c->type->kind === RustType::LIST ? RustType::list($vt) : RustType::map($c->type->params[0], $vt));
        }
        $s = $b->casts->convert($subj->code, $subj->type, RustType::str());
        if ($pat->type->kind === RustType::STR) {
            $r = $b->casts->convert($rep->code, $rep->type, RustType::str());
            return $b->narrow(new Val('preg_replace(&' . $pat->code . ', &' . $r . ', &' . $s . ', ' . $limit . ')?', RustType::str()), $call);
        }
        $pl = $b->casts->convert($pat->code, $pat->type, RustType::list(RustType::str()));
        if ($rep->type->kind === RustType::STR) {
            return $b->narrow(new Val('preg_replace_arr_s(&' . $pl . ', &' . $rep->code . ', &' . $s . ', ' . $limit . ')?', RustType::str()), $call);
        }
        $rl = $b->casts->convert($rep->code, $rep->type, RustType::list(RustType::str()));
        return $b->narrow(new Val('preg_replace_arr(&' . $pl . ', &' . $rl . ', &' . $s . ', ' . $limit . ')?', RustType::str()), $call);
    }

    private function f_preg_replace_callback(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $pat = $b->exprTo($args[0]->value, RustType::str());
        $s = $b->exprTo($args[2]->value, RustType::str());
        $limit = isset($args[3]) ? $b->exprTo($args[3]->value, RustType::int()) : '-1';
        $ct = $b->inferred($args[1]->value);
        $mt = $ct !== null && $ct->kind === RustType::CLOSURE && isset($ct->params[0]) ? $ct->params[0] : RustType::map(RustType::arrayKey(), RustType::str());
        $cb = $this->callback($b, $args[1]->value, [$mt], RustType::str());
        $conv = $b->casts->convert('__m', RustType::map(RustType::arrayKey(), RustType::str()), $mt);
        return $b->narrow(new Val('preg_replace_callback(&' . $pat . ', &' . $s . ', ' . $limit . ', { let __f = ' . $cb . '; move |__m: Map<ArrayKey, Str>| __f(' . $conv . ') })?', RustType::str()), $call);
    }

    private function f_preg_split(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $pat = $b->exprTo($args[0]->value, RustType::str());
        $s = $b->exprTo($args[1]->value, RustType::str());
        $limit = isset($args[2]) && !$b->isNullLiteral($args[2]->value) ? $b->exprTo($args[2]->value, RustType::int()) : '-1';
        $flags = isset($args[3]) ? $b->exprTo($args[3]->value, RustType::int()) : '0';
        $res = $b->inferredOrMixed($call);
        if ($flags !== '0' && str_contains($flags, 'OFFSET_CAPTURE')) {
            return $b->narrow(new Val('preg_split_offsets(&' . $pat . ', &' . $s . ', ' . $limit . ', ' . $flags . ')?', RustType::list(RustType::tuple([RustType::str(), RustType::int()]))), $call);
        }
        return $b->narrow(new Val('preg_split(&' . $pat . ', &' . $s . ', ' . $limit . ', ' . $flags . ')?', RustType::list(RustType::str())), $call);
    }

    private function f_preg_grep(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $pat = $b->exprTo($args[0]->value, RustType::str());
        $c = $this->container($b, $args[1]->value);
        $m = $b->casts->convert($c->code, $c->type, $c->type->kind === RustType::LIST ? RustType::map(RustType::int(), RustType::str()) : RustType::map($c->type->params[0], RustType::str()));
        $kt = $c->type->kind === RustType::LIST ? RustType::int() : $c->type->params[0];
        return new Val('preg_grep(&' . $pat . ', &' . $m . ')?', RustType::map($kt, RustType::str()));
    }

    // json

    private function f_json_encode(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $v = $b->exprTo($args[0]->value, RustType::mixed());
        $flags = isset($args[1]) ? $b->exprTo($args[1]->value, RustType::int()) : '0';
        $depth = isset($args[2]) ? $b->exprTo($args[2]->value, RustType::int()) : '512';
        return $b->narrow(new Val('json_encode(&' . $v . ', ' . $flags . ', ' . $depth . ')?', RustType::option(RustType::str())), $call);
    }

    private function f_json_decode(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $s = $b->exprTo($args[0]->value, RustType::str());
        $assoc = isset($args[1]) && !$b->isNullLiteral($args[1]->value) ? $b->exprTo($args[1]->value, RustType::bool()) : 'false';
        $depth = isset($args[2]) ? $b->exprTo($args[2]->value, RustType::int()) : '512';
        $flags = isset($args[3]) ? $b->exprTo($args[3]->value, RustType::int()) : '0';
        return $b->narrow(new Val('json_decode(&' . $s . ', ' . $assoc . ', ' . $depth . ', ' . $flags . ')?', RustType::mixed()), $call);
    }

    // misc

    private function f_assert(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $cond = $b->truthy($args[0]->value);
        $msg = isset($args[1]) ? $b->exprTo($args[1]->value, RustType::str()) : Names::strLit('assert(' . $args[0]->value->getType() . ')');
        return new Val('{ if !(' . $cond . ') { return Err(Throw::assertion(' . $msg . ')); } true }', RustType::bool());
    }

    private function f_func_get_args(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $b->warn('func_get_args', $call);
        return new Val('List::<Mixed>::new()', RustType::list(RustType::mixed()));
    }

    private function f_func_num_args(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $b->warn('func_num_args', $call);
        return new Val('0i64', RustType::int());
    }

    private function f_call_user_func(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $callee = $b->expr($args[0]->value);
        return $b->callValue($callee, array_slice($args, 1), $call);
    }

    private function f_call_user_func_array(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $callee = $b->exprTo($args[0]->value, RustType::dynCallable());
        $list = $b->exprTo($args[1]->value, RustType::list(RustType::mixed()));
        return $b->narrow(new Val($callee . '.call(' . $list . '.into_vec())?', RustType::mixed()), $call);
    }

    private function f_is_a_class(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->simple($b, $call, $args, ['is_a_name', ['&m', '&s', 'b=false'], 'b']);
    }

    private function f_class_alias(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $b->warn('class_alias', $call);
        return new Val('true', RustType::bool());
    }

    private function f_array_walk_recursive(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $b->warn('array_walk_recursive', $call);
        return new Val('true', RustType::bool());
    }

    private function f_array_map_keys(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->f_array_map($b, $call, $args);
    }

    private function f_iterator_to_array(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $v = $b->expr($args[0]->value);
        $preserve = !isset($args[1]) || $b->truthy($args[1]->value) !== 'false';
        $res = $b->inferredOrMixed($call);
        $t = $v->type;
        if ($t->kind === RustType::RT_GENERIC && ($t->name === 'Generator' || $t->name === 'PhpIterator' || $t->name === 'Traversable' || $t->name === 'IteratorAggregate')) {
            [$k, $val] = $t->params;
            if (!$preserve) {
                return new Val($v->code . '.into_list()', RustType::list($val));
            }
            $kk = $k->kind === RustType::INT ? RustType::int() : ($k->kind === RustType::STR ? RustType::str() : RustType::arrayKey());
            return new Val($v->code . '.into_map()', RustType::map($kk, $val));
        }
        if ($t->kind === RustType::CLASS_) {
            $cls = $b->program->classOf($t);
            $get = $cls !== null ? $b->program->findMethod($cls, 'getiterator') : null;
            if ($get !== null) {
                $inner = new Val($v->code . '.' . $get->rustName() . '()?', $get->return_type);
                $fake = new Expr\FuncCall($call->name, [new Arg(new Expr\Variable('__it')), ...array_slice($args, 1)], $call->getAttributes());
                $b->vars['__it'] = $get->return_type;
                $b->late['__it'] = false;
                $r = $this->f_iterator_to_array($b, $fake, $fake->getArgs());
                return new Val('{ let __it = ' . $inner->code . '; ' . $r->code . ' }', $r->type);
            }
        }
        if ($t->kind === RustType::LIST || $t->kind === RustType::MAP) {
            return $v;
        }
        $b->warn('iterator_to_array on ' . $t->toRust(), $call);
        return $b->dead('iterator_to_array', $res);
    }

    private function f_array_key_exists_str(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->f_array_key_exists($b, $call, $args);
    }

    private function f_unset(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return new Val('()', RustType::unit());
    }

    private function f_is_subclass_of(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $v = $b->expr($args[0]->value);
        $cls = $b->exprTo($args[1]->value, RustType::str());
        $vc = $v->type->kind === RustType::STR ? 'Mixed::Str(' . $v->code . ')' : $b->casts->convert($v->code, $v->type, RustType::mixed());
        return new Val('is_subclass_of_name(&' . $vc . ', &' . $cls . ')', RustType::bool());
    }

    private function f_is_a(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $v = $b->expr($args[0]->value);
        $cls = $b->exprTo($args[1]->value, RustType::str());
        $allow = isset($args[2]) ? $b->exprTo($args[2]->value, RustType::bool()) : 'false';
        $vc = $v->type->kind === RustType::STR ? 'Mixed::Str(' . $v->code . ')' : $b->casts->convert($v->code, $v->type, RustType::mixed());
        return new Val('is_a_name(&' . $vc . ', &' . $cls . ', ' . $allow . ')', RustType::bool());
    }

    private function f_get_class(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        if (!isset($args[0])) {
            return new Val(Names::strLit($b->class?->fqcn ?? ''), RustType::str());
        }
        $v = $b->expr($args[0]->value);
        if ($v->type->kind === RustType::CLASS_ || $v->type->kind === RustType::UNION || $v->type->kind === RustType::ANY_OBJECT) {
            return new Val('class_name_of(&' . $b->casts->convert($v->code, $v->type, RustType::mixed()) . ')', RustType::str());
        }
        return new Val('get_class(&' . $b->casts->convert($v->code, $v->type, RustType::mixed()) . ')', RustType::str());
    }

    private function f_spl_object_id(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $v = $b->expr($args[0]->value);
        if ($v->type->kind === RustType::CLASS_) {
            return new Val($v->code . '.obj_id() as i64', RustType::int());
        }
        return new Val('spl_object_id(&' . $b->casts->convert($v->code, $v->type, RustType::mixed()) . ')', RustType::int());
    }

    private function f_array_walk_keys(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->f_array_walk($b, $call, $args);
    }

    private function f_array_flip_keys(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->f_array_flip($b, $call, $args);
    }

    /** runtime hook: parsed XML document element as a nested array tree (null when malformed) */
    private function f___rt_xml_parse(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return new Val('php_rt::xml::xml_parse(&' . $b->exprTo($args[0]->value, RustType::str()) . ')', RustType::option(RustType::mixed()));
    }

    /** runtime hook: XML-escaped text (attribute context when the flag is set) */
    private function f___rt_xml_escape(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return new Val('php_rt::xml::xml_escape(&' . $b->exprTo($args[0]->value, RustType::str()) . ', ' . (isset($args[1]) ? $b->exprTo($args[1]->value, RustType::bool()) : 'false') . ')', RustType::str());
    }

    /** runtime hook: an instance of the named class without running its constructor (null if unknown) */
    private function f___rt_new_uninit(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return new Val('php_rt::registry::new_uninit(' . $b->exprTo($args[0]->value, RustType::str()) . '.as_bytes())', RustType::option(RustType::mixed()));
    }

    private function f_extract(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $b->warn('extract', $call);
        return new Val('0i64', RustType::int());
    }

    private function f_str_word_count(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->simple($b, $call, $args, ['str_word_count', ['&s'], 'i']);
    }

    private function f_array_key_exists_int(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->f_array_key_exists($b, $call, $args);
    }

    private function f___rt_tokenize(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $s = $b->exprTo($args[0]->value, RustType::str());
        return new Val('__rt_tokenize(&' . $s . ')', RustType::list(RustType::tuple([RustType::int(), RustType::str(), RustType::int(), RustType::int()])));
    }

    private function f_token_get_all(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $s = $b->exprTo($args[0]->value, RustType::str());
        return $b->narrow(new Val('token_get_all(&' . $s . ')', RustType::list(RustType::mixed())), $call);
    }

    private function f_debug_backtrace(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return new Val('List::<Mixed>::new()', RustType::list(RustType::mixed()));
    }

    private function __removed_f_simplexml_load_string(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $s = $b->exprTo($args[0]->value, RustType::str());
        return $b->narrow(new Val('simplexml_load_string(&' . $s . ')', RustType::option(RustType::class('SimpleXMLElement'))), $call);
    }

    private function __removed_f_simplexml_load_file(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $s = $b->exprTo($args[0]->value, RustType::str());
        return $b->narrow(new Val('simplexml_load_file(&' . $s . ')', RustType::option(RustType::class('SimpleXMLElement'))), $call);
    }

    private function __removed_f_simplexml_import_dom(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        $b->warn('simplexml_import_dom', $call);
        return new Val('None', RustType::option(RustType::class('SimpleXMLElement')));
    }

    private function f_array_walk_values(BodyEmitter $b, Expr\FuncCall $call, array $args): Val
    {
        return $this->f_array_walk($b, $call, $args);
    }

    /** Static calls on runtime-provided classes; null when the class is not a runtime class. */
    public function emitRuntimeStatic(BodyEmitter $b, string $lc_class, string $lc_method, array $args, Expr $site): ?Val
    {
        switch ($lc_class . '::' . $lc_method) {
            case 'weakreference::create':
                $v = $b->expr($args[0]->value);
                return new Val('WeakReference::create(' . $v->code . ')', RustType::rtGeneric('WeakReference', [$v->type]));
            case 'closure::fromcallable':
                return $b->expr($args[0]->value);
        }
        return null;
    }

    /** Method calls on runtime-provided generic types (SplObjectStorage, ArrayObject, Generator, closures...). */
    public function emitRuntimeMethod(BodyEmitter $b, Val $recv, string $name, array $args, Expr $site): Val
    {
        $t = $recv->type;
        $lc = strtolower($name);
        $argv = [];
        $res = $b->inferredOrMixed($site);
        if ($t->kind === RustType::DYN_CALLABLE) {
            if ($lc === '__invoke' || $lc === 'call') {
                return $b->callValue($recv, $args, $site);
            }
            if ($lc === 'bindto' || $lc === 'bind') {
                return new Val('{ let _ = ' . $b->expr($args[0]->value)->code . '; ' . $recv->code . ' }', $t);
            }
        }
        $params = $t->params;
        switch ($t->name . '::' . $lc) {
            case 'SplObjectStorage::attach':
            case 'WeakMap::offsetset':
                $k = $b->exprTo($args[0]->value, $params[0]);
                $v = isset($args[1]) ? $b->exprTo($args[1]->value, $params[1]) : $b->casts->defaultOf($params[1]);
                return new Val('{ ' . $recv->code . '.attach(' . $k . ', ' . $v . '); }', RustType::unit());
            case 'SplObjectStorage::detach':
            case 'SplObjectStorage::offsetunset':
            case 'WeakMap::offsetunset':
                return new Val('{ ' . $recv->code . '.detach(&' . $b->exprTo($args[0]->value, $params[0]) . '); }', RustType::unit());
            case 'SplObjectStorage::contains':
            case 'SplObjectStorage::offsetexists':
            case 'WeakMap::offsetexists':
                return new Val($recv->code . '.contains(&' . $b->exprTo($args[0]->value, $params[0]) . ')', RustType::bool());
            case 'SplObjectStorage::offsetget':
            case 'WeakMap::offsetget':
                return $b->narrow(new Val($recv->code . '.idx(&' . $b->exprTo($args[0]->value, $params[0]) . ')', $params[1]), $site);
            case 'SplObjectStorage::offsetset':
                $k = $b->exprTo($args[0]->value, $params[0]);
                $v = $b->exprTo($args[1]->value, $params[1]);
                return new Val('{ ' . $recv->code . '.attach(' . $k . ', ' . $v . '); }', RustType::unit());
            case 'SplObjectStorage::count':
            case 'ArrayObject::count':
            case 'ArrayIterator::count':
            case 'WeakMap::count':
                return new Val($recv->code . '.count()', RustType::int());
            case 'SplObjectStorage::getinfo':
                return new Val($recv->code . '.get_info()', $params[1]);
            case 'SplObjectStorage::addall':
                return new Val('{ ' . $recv->code . '.add_all(&' . $b->exprTo($args[0]->value, $t) . '); }', RustType::unit());
            case 'ArrayObject::getarraycopy':
            case 'ArrayIterator::getarraycopy':
                return new Val($recv->code . '.get_array_copy()', RustType::map($params[0]->kind === RustType::INT ? RustType::int() : ($params[0]->kind === RustType::STR ? RustType::str() : RustType::arrayKey()), $params[1]));
            case 'ArrayObject::offsetset':
            case 'ArrayIterator::offsetset':
                $k = $b->exprTo($args[0]->value, $params[0]);
                $v = $b->exprTo($args[1]->value, $params[1]);
                return new Val('{ ' . $recv->code . '.set(' . $k . ', ' . $v . '); }', RustType::unit());
            case 'ArrayObject::offsetget':
            case 'ArrayIterator::offsetget':
                return $b->narrow(new Val($recv->code . '.idx(&' . $b->exprTo($args[0]->value, $params[0]) . ')', $params[1]), $site);
            case 'ArrayObject::offsetexists':
            case 'ArrayIterator::offsetexists':
                return new Val($recv->code . '.contains(&' . $b->exprTo($args[0]->value, $params[0]) . ')', RustType::bool());
            case 'ArrayObject::offsetunset':
            case 'ArrayIterator::offsetunset':
                return new Val('{ ' . $recv->code . '.remove(&' . $b->exprTo($args[0]->value, $params[0]) . '); }', RustType::unit());
            case 'ArrayObject::append':
                return new Val('{ ' . $recv->code . '.append(' . $b->exprTo($args[0]->value, $params[1]) . '); }', RustType::unit());
            case 'ArrayObject::getiterator':
                return new Val($recv->code . '.get_iterator()', RustType::rtGeneric('ArrayIterator', $params));
            case 'Generator::current':
            case 'PhpIterator::current':
            case 'ArrayIterator::current':
                return $b->narrow(new Val($recv->code . '.current()', RustType::option($params[1])), $site);
            case 'Generator::key':
            case 'PhpIterator::key':
            case 'ArrayIterator::key':
                return $b->narrow(new Val($recv->code . '.key()', RustType::option($params[0])), $site);
            case 'Generator::next':
            case 'PhpIterator::next':
            case 'ArrayIterator::next':
                return new Val('{ ' . $recv->code . '.next(); }', RustType::unit());
            case 'Generator::rewind':
            case 'PhpIterator::rewind':
            case 'ArrayIterator::rewind':
                return new Val('{ ' . $recv->code . '.rewind(); }', RustType::unit());
            case 'Generator::valid':
            case 'PhpIterator::valid':
            case 'ArrayIterator::valid':
                return new Val($recv->code . '.valid()', RustType::bool());
            case 'Generator::send':
                return new Val('{ let _ = ' . $b->expr($args[0]->value)->code . '; ' . $recv->code . '.next(); ' . $recv->code . '.current() }', RustType::option($params[1]));
            case 'Generator::getreturn':
                return new Val('Mixed::Null', RustType::mixed());
            case 'WeakReference::get':
                return new Val($recv->code . '.get()', RustType::option($params[0]));
            case 'StdClass::__get':
                return new Val($recv->code . '.get(' . $b->exprTo($args[0]->value, RustType::str()) . ')', RustType::option(RustType::mixed()));
        }
        $b->warn('unknown runtime method ' . $t->name . '::' . $name, $site);
        return $b->dead('runtime method ' . $t->name . '::' . $name . '', $res);
    }
}

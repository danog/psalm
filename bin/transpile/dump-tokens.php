<?php
// Dumps token_get_all() output for the given files as JSON lines: {"file":..., "tokens":[[id,text,line],...]}
declare(strict_types=1);
$out = fopen($argv[1], 'w');
foreach (array_slice($argv, 2) as $file) {
    $code = file_get_contents($file);
    $codes = [$code];
    if (str_ends_with($file, '.test')) {
        // php-parser test files: sections separated by ----- lines; take chunks containing <?php
        $codes = array_values(array_filter(preg_split('/^-----\R/m', $code), fn($c) => str_contains($c, '<?php')));
    }
    foreach ($codes as $i => $c) {
        $tokens = [];
        foreach (@token_get_all($c) as $t) {
            $tokens[] = is_array($t) ? [token_name($t[0]), $t[1], $t[2]] : [$t, $t, -1];
        }
        fwrite($out, json_encode(['file' => $file . '#' . $i, 'code' => $c, 'tokens' => $tokens], JSON_INVALID_UTF8_SUBSTITUTE) . "\n");
    }
}

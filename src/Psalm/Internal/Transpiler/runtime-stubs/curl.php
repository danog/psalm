<?php

/** The compiled analyzer has no HTTP client: cURL handles cannot be created (Shepherd reporting is skipped). */
final class CurlHandle
{
}

function curl_init(?string $url = null): CurlHandle|false
{
    return false;
}

/** @param string|int|bool|list<string> $value */
function curl_setopt(CurlHandle $handle, int $option, string|int|bool|array $value): bool
{
    return false;
}

function curl_exec(CurlHandle $handle): string|bool
{
    return false;
}

/** @return array{http_code: int, ssl_verify_result: int} */
function curl_getinfo(CurlHandle $handle, ?int $option = null): array
{
    return ['http_code' => 0, 'ssl_verify_result' => 0];
}

function curl_close(CurlHandle $handle): void
{
}

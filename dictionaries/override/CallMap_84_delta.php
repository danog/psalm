<?php // phpcs:ignoreFile

return array (
  'added' => 
  array (
  ),
  'changed' => 
  array (
    'exit' => 
    array (
      'old' => 
      array (
        0 => 'mixed',
        'status' => 'int|string',
      ),
      'new' => 
      array (
        0 => 'mixed',
        'status=' => 'int|string',
      ),
    ),
    'openssl_csr_sign' => 
    array (
      'old' => 
      array (
        0 => 'OpenSSLCertificate|false',
        'csr' => 'OpenSSLCertificateSigningRequest|string',
        'ca_certificate' => 'OpenSSLCertificate|null|string',
        'private_key' => 'OpenSSLAsymmetricKey|OpenSSLCertificate|list{OpenSSLAsymmetricKey|OpenSSLCertificate|string, string}|string',
        'days' => 'int',
        'options=' => 'array<array-key, mixed>|null',
        'serial=' => 'int',
      ),
      'new' => 
      array (
        0 => 'OpenSSLCertificate|false',
        'csr' => 'OpenSSLCertificateSigningRequest|string',
        'ca_certificate' => 'OpenSSLCertificate|null|string',
        'private_key' => 'OpenSSLAsymmetricKey|OpenSSLCertificate|list{OpenSSLAsymmetricKey|OpenSSLCertificate|string, string}|string',
        'days' => 'int',
        'options=' => 'array<array-key, mixed>|null',
        'serial=' => 'int',
        'serial_hex=' => 'null|string',
      ),
    ),
    'pg_select' => 
    array (
      'old' => 
      array (
        0 => 'array<array-key, mixed>|false|string',
        'connection' => 'PgSql\\Connection',
        'table_name' => 'string',
        'conditions' => 'array<array-key, mixed>',
        'flags=' => 'int',
        'mode=' => 'int',
      ),
      'new' => 
      array (
        0 => 'array<array-key, mixed>|false|string',
        'connection' => 'PgSql\\Connection',
        'table_name' => 'string',
        'conditions=' => 'array<array-key, mixed>',
        'flags=' => 'int',
        'mode=' => 'int',
      ),
    ),
  ),
  'removed' => 
  array (
  ),
);
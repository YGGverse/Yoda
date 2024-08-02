<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Interface\Model;

use \OpenSSLAsymmetricKey;
use \OpenSSLCertificate;
use \OpenSSLCertificateSigningRequest;

/*
 * Certificate-based Identity API
 *
 */
interface Identity
{
    /*
     * Industry standards recommend limiting the validity period of public SSL/TLS certificates to 397 days.
     * This is a guideline default value provided by the CA/B Forum to improve security and manage risk.
     */
    public const CSR_SIGN_DAYS = 397;

    public const PRIVATE_KEY_BITS = 2048;
    public const PRIVATE_KEY_TYPE = OPENSSL_KEYTYPE_RSA;

    // Generate new private key
    public static function new(
        int $bits = self::PRIVATE_KEY_BITS,
        int $type = self::PRIVATE_KEY_TYPE
    ): OpenSSLAsymmetricKey;

    // Generate certificate signing request (CSR)
    public static function csr(
        OpenSSLAsymmetricKey $key
    ): OpenSSLCertificateSigningRequest;

    // Sign the CSR
    public static function sign(
        OpenSSLCertificateSigningRequest $csr,
        OpenSSLCertificate|OpenSSLAsymmetricKey|array|string $key,
        OpenSSLCertificate|string|null $crt = null, // self-signed
        int $days = self::CSR_SIGN_DAYS
    ):  OpenSSLCertificate;

    // Read certificate
    public static function read(
        OpenSSLCertificate|string $crt
    ): OpenSSLCertificate;

    // Dump certificate
    public static function parse(
        OpenSSLCertificate|string $crt
    ): array;
}
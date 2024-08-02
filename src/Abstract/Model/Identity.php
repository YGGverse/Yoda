<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Model;

use \Exception;
use \OpenSSLAsymmetricKey;
use \OpenSSLCertificate;
use \OpenSSLCertificateSigningRequest;

abstract class Identity implements \Yggverse\Yoda\Interface\Model\Identity
{
    // Generate a new private key
    public static function new(
        int $bits = self::PRIVATE_KEY_BITS,
        int $type = self::PRIVATE_KEY_TYPE
    ): OpenSSLAsymmetricKey
    {
        $key = openssl_pkey_new(
            [
                'private_key_bits' => $bits,
                'private_key_type' => $type
            ]
        );

        if ($key)
        {
            return $key;
        }

        throw new Exception;
    }

    // Generate a new certificate signing request (CSR)
    public static function csr(
        OpenSSLAsymmetricKey $key
    ): OpenSSLCertificateSigningRequest
    {
        $csr = openssl_csr_new(
            [
                // 'commonName' => $commonName @TODO
            ],
            $key
        );

        if ($csr)
        {
            return $csr;
        }

        throw new Exception;
    }

    // Sign the CSR
    public static function sign(
        OpenSSLCertificateSigningRequest $csr,
        OpenSSLCertificate|OpenSSLAsymmetricKey|array|string $key,
        OpenSSLCertificate|string|null $crt = null, // self-signed
        int $days = self::CSR_SIGN_DAYS
    ):  OpenSSLCertificate
    {
        $x509 = openssl_csr_sign(
            $csr,
            $crt,
            $key,
            $days
        );

        if ($x509)
        {
            return $x509;
        }

        throw new Exception;
    }

    // Read certificate
    public static function read(
        OpenSSLCertificate|string $crt
    ): OpenSSLCertificate
    {
        if ($result = openssl_x509_read($crt))
        {
            return $result;
        }

        throw new Exception;
    }

    // Dump certificate
    public static function parse(
        OpenSSLCertificate|string $crt
    ): array
    {
        if ($result = openssl_x509_parse($crt))
        {
            return $result;
        }

        throw new Exception;
    }
}
<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Identity;

use \Exception;
use \OpenSSLAsymmetricKey;
use \OpenSSLCertificate;

class Gemini extends \Yggverse\Yoda\Abstract\Model\Identity
{
    // Update defaults
    public const CSR_SIGN_DAYS = 365 * 1965; // @TODO

    // Init identity variables
    protected OpenSSLAsymmetricKey $_key;
    protected OpenSSLCertificate $_crt;

    // Init new identity
    public function __construct(
        ?OpenSSLAsymmetricKey $key = null,
        ?OpenSSLCertificate $crt = null
    ) {
        // Init private key
        $this->_key = $key ? $key : self::new();

        // Init self-signed certificate
        $this->_crt = $crt ? $crt : self::sign(
            self::csr(
                $this->_key
            ),
            $this->_key,
            null,
            self::CSR_SIGN_DAYS
        );
    }

    // Get certificate
    public function crt(
        ?OpenSSLCertificate $crt = null
    ): string
    {
        $pem = '';

        $result = openssl_x509_export(
            $crt ? $crt : $this->_crt,
            $pem
        );

        if ($result)
        {
            return $pem;
        }

        throw new Exception;
    }

    // Get private key
    public function key(
        ?OpenSSLAsymmetricKey $key = null
    ): string
    {
        $pem = '';

        $result = openssl_pkey_export(
            $key ? $key : $this->_key,
            $pem
        );

        if ($result)
        {
            return $pem;
        }

        throw new Exception;
    }
}
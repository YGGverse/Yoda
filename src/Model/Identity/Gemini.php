<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Identity;

use \Exception;
use \OpenSSLAsymmetricKey;
use \OpenSSLCertificate;

class Gemini extends \Yggverse\Yoda\Abstract\Model\Identity
{
    // Init defaults
    public const CSR_SIGN_TIME = 253402300799; // U 9999-12-31 23:59:59

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
            intval(
                floor( // calculate max possible identity age, days
                    (self::CSR_SIGN_TIME - time()) / (60 * 60 * 24)
                )
            )
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
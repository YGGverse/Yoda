<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Identity;

use \Exception;
use \OpenSSLAsymmetricKey;
use \OpenSSLCertificate;

use \Yggverse\Net\Address;

use \Yggverse\Yoda\Model\Database;

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

    /**
     * Return identity match Address | NULL
     *
     * https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates
     *
     */
    public static function match(
        Address $address,
        Database $database,
        array $identities = []
    ): ?object
    {
        foreach (
            // Select host records
            $database->auth->like(
                sprintf(
                    '%s%%',
                    $address->get(
                        true,
                        true,
                        true,
                        true,
                        true,
                        false,
                        false,
                        false
                    )
                )
            ) as $auth
        ) {
            // Parse result address
            $request = new Address(
                $auth->request
            );

            // Filter results match current path prefix
            if (str_starts_with($address->getPath(), $request->getPath()))
            {
                $identities[
                    $auth->identity
                ] = $auth->request;
            }
        }

        // Results found
        if ($identities)
        {
            uasort( // max-level
                $identities,
                function ($a, $b)
                {
                    return mb_strlen($b) <=> mb_strlen($a);
                }
            );

            return $database->identity->get(
                intval(
                    array_key_first(
                        $identities
                    )
                )
            );
        }

        return null;
    }
}
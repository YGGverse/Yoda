use gtk::glib::DateTime;
use openssl::{
    asn1::{Asn1Integer, Asn1Time},
    bn::{BigNum, MsbOption},
    hash::MessageDigest,
    pkey::PKey,
    rsa::Rsa,
    x509::{X509Builder, X509Name},
};
use std::error::Error;

// Defaults

pub const VERSION: i32 = 0; // 1 https://docs.openssl.org/master/man3/X509_get_version
pub const BITS: u32 = 2048;

/// Generate new Gemini certificate
/// * return PEM string as `Result`
pub fn generate(
    time: (DateTime, DateTime), // valid (from, to)
    name: &str,
) -> Result<String, Box<dyn Error>> {
    // Generate new RSA key pair
    let rsa = Rsa::generate(BITS)?;
    let key = PKey::from_rsa(rsa)?;

    // Init X509 name
    let mut name_builder = X509Name::builder()?;
    name_builder.append_entry_by_text("CN", name)?;
    let name = name_builder.build();

    // Init serial number
    let mut number = BigNum::new()?;
    number.rand(128, MsbOption::MAYBE_ZERO, true)?;
    let serial = Asn1Integer::from_bn(&number)?;

    // Init validity period
    let not_before = Asn1Time::from_unix(time.0.to_unix())?;
    let not_after = Asn1Time::from_unix(time.1.to_unix())?;

    // Build
    let mut builder = X509Builder::new()?;

    builder.set_version(VERSION)?;
    builder.set_serial_number(&serial)?;
    builder.set_subject_name(&name)?;
    builder.set_issuer_name(&name)?;
    builder.set_not_before(&not_before)?;
    builder.set_not_after(&not_after)?;
    builder.set_pubkey(&key)?;
    builder.sign(&key, MessageDigest::sha256())?;

    let certificate = builder.build();

    // Result
    Ok(format!(
        "{}{}",
        String::from_utf8(certificate.to_pem()?)?,
        String::from_utf8(key.private_key_to_pem_pkcs8()?)?,
    ))
}

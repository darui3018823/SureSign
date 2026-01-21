use anyhow::Result;
use rcgen::{Certificate, CertificateParams, DnType, KeyPair, SanType};
use std::net::IpAddr;
use std::str::FromStr;
use time::{Duration, OffsetDateTime}; // rcgen uses time crate

#[derive(Debug, Clone, Copy)]
pub enum KeyType {
    Rsa,
    Ecdsa,
    Ed25519,
}

pub struct CertOptions {
    // Simple
    pub cn: String,
    pub sans: Vec<String>,
    pub validity_days: i64,
    // Full
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub organization: Option<String>,
    pub org_unit: Option<String>,
    // All
    pub key_type: KeyType,
}

pub struct GeneratedCert {
    pub cert_pem: String,
    pub key_pem: String,
    pub pfx: Vec<u8>,
}

pub fn generate_cert(opt: CertOptions) -> Result<GeneratedCert> {
    let mut params = CertificateParams::new(vec![opt.cn.clone()]);

    // Set validity
    let now = OffsetDateTime::now_utc();
    let end = now + Duration::days(opt.validity_days);
    params.not_before = now;
    params.not_after = end;

    // Set SANs
    for san in opt.sans {
        if let Ok(ip) = IpAddr::from_str(&san) {
            params.subject_alt_names.push(SanType::IpAddress(ip));
        } else {
            params.subject_alt_names.push(SanType::DnsName(san));
        }
    }

    // Set DN fields (Full Mode)
    if let Some(c) = opt.country {
        params.distinguished_name.push(DnType::CountryName, c);
    }
    if let Some(st) = opt.state {
        params
            .distinguished_name
            .push(DnType::StateOrProvinceName, st);
    }
    if let Some(l) = opt.city {
        params.distinguished_name.push(DnType::LocalityName, l);
    }
    if let Some(o) = opt.organization {
        params.distinguished_name.push(DnType::OrganizationName, o);
    }
    if let Some(ou) = opt.org_unit {
        params
            .distinguished_name
            .push(DnType::OrganizationalUnitName, ou);
    }

    // Set Key Algorithm (All Mode)
    // rcgen 0.11 uses alg when generating KeyPair.
    let key_pair = match opt.key_type {
        KeyType::Rsa => KeyPair::generate(&rcgen::PKCS_RSA_SHA256)?,
        KeyType::Ecdsa => KeyPair::generate(&rcgen::PKCS_ECDSA_P256_SHA256)?,
        KeyType::Ed25519 => KeyPair::generate(&rcgen::PKCS_ED25519)?,
    };
    params.key_pair = Some(key_pair);

    // Generate Certificate
    let cert = Certificate::from_params(params)?;
    let cert_pem = cert.serialize_pem()?;
    let key_pem = cert.serialize_private_key_pem();

    // Generate PFX
    let cert_der = cert.serialize_der()?;
    let key_der = cert.serialize_private_key_der();

    let pfx = p12::PFX::new(&cert_der, &key_der, None, "", &opt.cn)
        .ok_or_else(|| anyhow::anyhow!("Failed to generate PFX struct"))?;

    let pfx_bytes = pfx.to_der();

    Ok(GeneratedCert {
        cert_pem,
        key_pem,
        pfx: pfx_bytes,
    })
}

use anyhow::{Context, Result};
use rcgen::{Certificate, CertificateParams, SanType};
use std::net::IpAddr;
use std::str::FromStr;
use time::{Duration, OffsetDateTime}; // rcgen uses time crate

pub struct CertOptions {
    pub cn: String,
    pub sans: Vec<String>,
    pub validity_days: i64,
}

pub struct GeneratedCert {
    pub cert_pem: String,
    pub key_pem: String,
    pub pfx: Vec<u8>,
}

pub fn generate_cert(opt: CertOptions) -> Result<GeneratedCert> {
    let mut params = CertificateParams::new(vec![opt.cn.clone()]);
    
    // Set validity
    // rcgen uses time::OffsetDateTime
    let now = OffsetDateTime::now_utc();
    let end = now + Duration::days(opt.validity_days);
    params.not_before = now;
    params.not_after = end;

    // Set SANs
    for san in opt.sans {
        if let Ok(ip) = IpAddr::from_str(&san) {
            params.subject_alt_names.push(SanType::IpAddress(ip));
        } else {
            params.subject_alt_names.push(SanType::DnsName(san.into()));
        }
    }

    // Generate KeyPair and Certificate
    let cert = Certificate::from_params(params)?;
    let cert_pem = cert.serialize_pem()?;
    // rcgen 0.11+ separates key generation if needed, but from_params generates a keypair internally unless provided.
    // wait, from_params creates a Certificate which contains the keypair.
    let key_pem = cert.serialize_private_key_pem();

    // Generate PFX
    // p12 crate requires DER cert and key
    let cert_der = cert.serialize_der()?;
    // We need the private key in DER too.
    let key_der = cert.serialize_private_key_der();

    // p12::PFX::new returns PFX struct in version 0.6.3
    // We need to verify if it has .to_der()
    // It seems commonly PFX::new(...) returns Option<Vec<u8>> in some versions or PFX struct in others.
    // Based on error "expected Vec<u8>, found PFX", it returns PFX.
    let pfx = p12::PFX::new(
        &cert_der,
        &key_der,
        None,
        "",
        &opt.cn,
    )
    .ok_or_else(|| anyhow::anyhow!("Failed to generate PFX struct"))?;
    
    let pfx_bytes = pfx.to_der();

    Ok(GeneratedCert {
        cert_pem,
        key_pem,
        pfx: pfx_bytes,
    })
}

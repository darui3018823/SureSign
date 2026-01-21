use anyhow::Result;
use rcgen::{CertificateParams, DnType, KeyPair, SanType};
use std::net::IpAddr;
use std::str::FromStr;
use time::{Duration, OffsetDateTime};

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
    // New: PFX password
    pub pfx_password: String,
}

pub struct GeneratedCert {
    pub cert_pem: String,
    pub key_pem: String,
    pub pfx: Vec<u8>,
}

pub fn generate_cert(opt: CertOptions) -> Result<GeneratedCert> {
    // Generate key pair based on key type
    let key_pair = match opt.key_type {
        KeyType::Rsa => KeyPair::generate_for(&rcgen::PKCS_RSA_SHA256)?,
        KeyType::Ecdsa => KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256)?,
        KeyType::Ed25519 => KeyPair::generate_for(&rcgen::PKCS_ED25519)?,
    };

    // Create certificate params
    let mut params = CertificateParams::new(vec![opt.cn.clone()])?;

    // Set validity
    let now = OffsetDateTime::now_utc();
    let end = now + Duration::days(opt.validity_days);
    params.not_before = now;
    params.not_after = end;

    // Set SANs
    for san in &opt.sans {
        if let Ok(ip) = IpAddr::from_str(san) {
            params.subject_alt_names.push(SanType::IpAddress(ip));
        } else {
            params
                .subject_alt_names
                .push(SanType::DnsName(san.clone().try_into()?));
        }
    }

    // Set DN fields (Full Mode)
    if let Some(c) = &opt.country {
        params.distinguished_name.push(DnType::CountryName, c);
    }
    if let Some(st) = &opt.state {
        params
            .distinguished_name
            .push(DnType::StateOrProvinceName, st);
    }
    if let Some(l) = &opt.city {
        params.distinguished_name.push(DnType::LocalityName, l);
    }
    if let Some(o) = &opt.organization {
        params.distinguished_name.push(DnType::OrganizationName, o);
    }
    if let Some(ou) = &opt.org_unit {
        params
            .distinguished_name
            .push(DnType::OrganizationalUnitName, ou);
    }

    // Generate self-signed certificate
    let cert = params.self_signed(&key_pair)?;
    let cert_pem = cert.pem();
    let key_pem = key_pair.serialize_pem();

    // Generate PFX
    let cert_der = cert.der().as_ref();
    let key_der = key_pair.serialize_der();

    let pfx_password: Option<&[u8]> = if opt.pfx_password.is_empty() {
        None
    } else {
        Some(opt.pfx_password.as_bytes())
    };

    let pfx = p12::PFX::new(cert_der, &key_der, pfx_password, "", &opt.cn)
        .ok_or_else(|| anyhow::anyhow!("Failed to generate PFX struct"))?;

    let pfx_bytes = pfx.to_der();

    Ok(GeneratedCert {
        cert_pem,
        key_pem,
        pfx: pfx_bytes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_options() -> CertOptions {
        CertOptions {
            cn: "localhost".to_string(),
            sans: vec!["127.0.0.1".to_string()],
            validity_days: 365,
            country: None,
            state: None,
            city: None,
            organization: None,
            org_unit: None,
            key_type: KeyType::Ecdsa,
            pfx_password: String::new(),
        }
    }

    #[test]
    fn test_generate_ecdsa_cert() {
        let opt = default_options();
        let result = generate_cert(opt);
        assert!(result.is_ok());
        let cert = result.unwrap();
        assert!(cert.cert_pem.contains("BEGIN CERTIFICATE"));
        assert!(cert.key_pem.contains("BEGIN PRIVATE KEY"));
        assert!(!cert.pfx.is_empty());
    }

    #[test]
    fn test_generate_ed25519_cert() {
        let mut opt = default_options();
        opt.key_type = KeyType::Ed25519;
        let result = generate_cert(opt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_with_full_dn() {
        let mut opt = default_options();
        opt.country = Some("JP".to_string());
        opt.state = Some("Tokyo".to_string());
        opt.city = Some("Shibuya".to_string());
        opt.organization = Some("Test Corp".to_string());
        opt.org_unit = Some("Dev".to_string());
        let result = generate_cert(opt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_with_multiple_sans() {
        let mut opt = default_options();
        opt.sans = vec![
            "127.0.0.1".to_string(),
            "192.168.1.1".to_string(),
            "example.local".to_string(),
        ];
        let result = generate_cert(opt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_with_pfx_password() {
        let mut opt = default_options();
        opt.pfx_password = "testpassword".to_string();
        let result = generate_cert(opt);
        assert!(result.is_ok());
    }
}

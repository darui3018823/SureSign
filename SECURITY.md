# Security Policy

> 📖 **Languages:** [English](./SECURITY.md) | [日本語](./japanese/SECURITY_ja.md)

## Reporting a Vulnerability

**Please do not open a public issue for security vulnerabilities.**

If you discover a security vulnerability in SureSign, please email us at [security@daruks.com](mailto:security@daruks.com) with:

- Description of the vulnerability
- Steps to reproduce (if applicable)
- Potential impact
- Suggested fix (if you have one)

We will:
1. Acknowledge receipt of your report within 48 hours
2. Investigate the vulnerability
3. Develop and test a fix
4. Release a patched version as soon as possible
5. Credit you in the security advisory (unless you prefer to remain anonymous)

## Supported Versions

| Version | Status |
|---------|--------|
| v1.0.x  | Actively supported |

We recommend always using the latest version for security updates.

## Security Best Practices

When using SureSign:

- Keep your Rust and dependencies up to date
- Review the code before running it in production
- Use appropriate file permissions for certificates and keys
- Follow best practices for cryptographic key management
- Report any suspicious activity or potential vulnerabilities

## Cryptographic Standards

SureSign uses:
- **RSA**: For digital signatures (RSA-2048 minimum recommended)
- **AES-256**: For encryption (when applicable)
- **SHA-256**: For hashing

All cryptographic operations use well-established libraries (rcgen, p12).

## Dependencies

We regularly review and update our dependencies to ensure security patches are applied. You can check dependencies with:

```bash
cargo tree
cargo audit
```

## Vulnerability Disclosure Timeline

We follow responsible disclosure practices:
- **Day 0**: Vulnerability reported
- **Day 1-7**: Initial assessment and verification
- **Day 7-30**: Development and testing of fix
- **Day 30+**: Public disclosure and release

Timelines may vary based on severity and complexity.

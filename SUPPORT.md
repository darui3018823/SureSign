# Support

> 📖 **Languages:** [English](./SUPPORT.md) | [日本語](./japanese/SUPPORT_ja.md)

Thank you for using SureSign! Here are the ways you can get help and support.

## Documentation

- [README](./README.md) - Project overview and quick start
- [CONTRIBUTING](./CONTRIBUTING.md) - Contribution guidelines
- [Code of Conduct](./Code_of_Conduct.md) - Community standards

## Getting Help

### GitHub Issues

For bugs and feature requests, please use [GitHub Issues](https://github.com/darui3018823/SureSign/issues):

1. Check existing issues to avoid duplicates
2. Provide clear description, steps to reproduce, and environment details
3. Include error messages or logs when applicable

### Email Support

For other inquiries:
- General questions: [contact@daruks.com](mailto:contact@daruks.com)
- Security issues: [security@daruks.com](mailto:security@daruks.com)

## Frequently Asked Questions

### Installation Issues

**Q: How do I build SureSign from source?**

A: Run the following commands:
```bash
git clone https://github.com/darui3018823/SureSign.git
cd SureSign
cargo build --release
```

**Q: What are the system requirements?**

A: 
- Rust 1.70 or later
- 50 MB disk space
- No external dependencies for runtime

### Usage Issues

**Q: Where are generated certificates stored?**

A: By default, certificates are stored in the current working directory. You can specify a custom path using command-line arguments.

**Q: Can I use SureSign in production?**

A: SureSign v1.0.0 is production-ready. Always review the security documentation and follow best practices for certificate management.

## Community

- **Discussions**: Use GitHub Discussions for general questions and ideas
- **Pull Requests**: We welcome contributions! See [CONTRIBUTING.md](./CONTRIBUTING.md)
- **Social Media**: Follow updates on relevant platforms

## Reporting Issues

### Types of Issues We Accept

- Bug reports with reproducible steps
- Feature requests with clear use cases
- Documentation improvements
- Performance improvements
- Security vulnerabilities (privately)

### Issue Template

When reporting issues, please include:

1. **SureSign Version**: `suresign --version`
2. **OS and Environment**: Windows/Linux/macOS, architecture
3. **Error Message**: Full error output
4. **Steps to Reproduce**: Clear, numbered steps
5. **Expected Behavior**: What should happen
6. **Actual Behavior**: What actually happened

## Response Times

We aim to:
- Acknowledge issues within 48 hours
- Provide initial assessment within 1 week
- Release fixes for critical bugs within 2 weeks

Response times may vary based on complexity and team availability.

## Roadmap

For planned features and improvements, check our [GitHub Projects](https://github.com/darui3018823/SureSign/projects).

## Credits

SureSign is built with:
- [Rust](https://www.rust-lang.org/)
- [clap](https://github.com/clap-rs/clap) - CLI parsing
- [rcgen](https://github.com/rustls/rcgen) - Certificate generation
- [p12](https://github.com/tintinn/p12) - PKCS#12 support

## License

SureSign is licensed under the [BSD 2-Clause License](./license).

---

Thank you for using SureSign! We appreciate your feedback and contributions.

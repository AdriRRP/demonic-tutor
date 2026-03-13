# Security Policy

## Reporting Vulnerabilities

This project is in early development. If you discover a security vulnerability, please report it responsibly.

**Do not disclose publicly until triaged.**

### How to Report

Use GitHub's [private vulnerability reporting](https://github.com/AdriRRP/demonic-tutor/security/advisories/new) to report security issues. This allows for responsible disclosure without public exposure before a fix is available.

Alternatively, open a GitHub issue with a note that it contains a security concern, and do not include sensitive details in the issue body.

### Scope

- Domain logic vulnerabilities
- Dependency vulnerabilities (detected by `cargo audit`)
- Any issue that could affect users in production

### Response Timeline

As an early-stage project, response times may vary. We aim to acknowledge reports within 7 days and provide a timeline for fixes.

## Security Practices

- `cargo audit` runs in CI to catch dependency vulnerabilities
- Dependencies are updated via Dependabot
- Production code is panic-free
- Strict clippy linting enforced

# Security Policy — DemonicTutor

## Reporting Vulnerabilities

If you discover a security vulnerability in DemonicTutor, please report it responsibly.

**Do not disclose the issue publicly until it has been reviewed.**

### Preferred Reporting Method

Use GitHub's private vulnerability reporting:

https://github.com/AdriRRP/demonic-tutor/security/advisories/new

This allows the issue to be discussed privately before a fix is published.

### Alternative

If private reporting is not possible, open a GitHub issue and clearly mark it as a **security concern**.  
Do not include sensitive details in the public issue.

---

# Security Scope

Security reports may include issues such as:

- vulnerabilities that could compromise user data
- unsafe handling of external inputs
- dependency vulnerabilities affecting the application
- flaws that could allow malicious manipulation of gameplay state
- vulnerabilities in WebAssembly integration or browser execution

Reports about general bugs or incorrect gameplay behavior should be filed as normal issues unless they have clear security implications.

---

# Threat Model (Current Stage)

DemonicTutor is primarily a **client-side application**.

The system:

- runs in the browser
- executes core logic through WebAssembly
- does not currently operate a centralized backend service
- does not manage user accounts or private user data

Because of this architecture, the primary security concerns involve:

- malicious inputs affecting gameplay state
- dependency vulnerabilities
- unsafe browser integration
- potential denial-of-service scenarios

---

# Response Process

As an early-stage open-source project, response times may vary.

The general process is:

1. Acknowledge the report
2. Evaluate the issue and confirm impact
3. Develop and test a fix
4. Publish the fix and advisory if necessary

We aim to acknowledge reports within **7 days**.

---

# Security Practices

The project applies several practices to reduce risk:

- `cargo audit` runs in CI to detect dependency vulnerabilities
- dependencies are monitored and updated via Dependabot
- strict clippy linting is enforced
- panic-free production code is preferred
- domain logic is deterministic and testable in isolation

These practices aim to reduce both runtime failures and potential attack surfaces.

---

# Out of Scope

The following typically do **not** qualify as security vulnerabilities:

- gameplay rule inaccuracies
- incomplete rules coverage
- modeling simplifications
- performance issues without security implications

These should be reported as normal issues instead.

---

# Responsible Disclosure

Contributors and researchers are encouraged to follow responsible disclosure practices.

Security issues should not be publicly disclosed until a fix or mitigation is available whenever possible.

# Security Auditing Guide

## Running Cargo Audit

Explog CMS recommends running security audits regularly to check for vulnerabilities in dependencies.

### Installation

```bash
cargo install cargo-audit
```

### Running Audit

```bash
# From project root
cargo audit
```

### CI Integration

Add to your CI pipeline:

```yaml
# GitHub Actions example
- name: Security audit
  run: cargo audit
```

### Recommended Frequency

- Before each release
- Weekly for active development
- After dependency updates

## Dependency Review

When updating dependencies, always:
1. Check CHANGELOG of updated packages
2. Run `cargo audit` after updates
3. Review security advisories at https://rustsec.org/

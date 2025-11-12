# Security Checklist for Contributors

## Before Committing Code
- [ ] Review code for secrets (API keys, private keys) – use environment variables or vaults.
- [ ] Ensure all inputs are validated and sanitized to prevent injection attacks.
- [ ] Check for proper error handling without leaking sensitive information.

## Key Management
- [ ] Use Ed25519 keys for node identity and attestation.
- [ ] Store keys securely (e.g., hardware wallets or encrypted storage).
- [ ] Rotate keys regularly and revoke compromised ones.
- [ ] Follow rotation procedures: Generate new key, update configs, test, then delete old key.

## RBAC and Access Control
- [ ] Implement role-based access for APIs and services.
- [ ] Use least privilege principle for user permissions.
- [ ] Audit access logs regularly.
- [ ] Validate RBAC manifests in CI before deployment.

## Threat Model Updates
- [ ] Update threat model in docs/security.md for new features.
- [ ] Conduct security reviews for changes affecting trust boundaries.
- [ ] Run automated security scans (e.g., SAST, DAST).

## Deployment Security
- [ ] Use TLS/Noise for all network communications.
- [ ] Harden containers (non-root users, minimal images).
- [ ] Enable monitoring and alerting for anomalies.
- [ ] For Helm/Terraform: Use secrets management (e.g., Vault), encrypt state files, restrict provider access.

## Compliance
- [ ] Ensure GDPR/CCPA compliance for user data handling.
- [ ] Document data retention and deletion policies.
- [ ] Conduct regular compliance audits.
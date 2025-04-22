# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability within webull-rs, please send an email to wszzsw@nullified.xyz. All security vulnerabilities will be promptly addressed.

Please include the following information in your report:

- Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit the issue

This information will help us triage your report more quickly.

## Security Best Practices

When using webull-rs in your applications, please follow these security best practices:

1. **Secure Credential Storage**: Use the provided `EncryptedCredentialStore` to securely store credentials, or implement your own secure storage mechanism.

2. **API Key Protection**: Never hardcode API keys or secrets in your application code. Use environment variables or a secure configuration management system.

3. **Regular Updates**: Keep webull-rs and its dependencies up to date to benefit from security patches.

4. **Rate Limiting**: Use the built-in rate limiting functionality to avoid API rate limit violations.

5. **Error Handling**: Properly handle errors and avoid exposing sensitive information in error messages.

6. **Logging**: Be careful not to log sensitive information such as API keys, tokens, or user credentials.

7. **MFA**: Use multi-factor authentication when available.

8. **TLS**: Ensure that all API requests are made over HTTPS.

9. **Token Management**: Implement proper token rotation and expiration handling.

10. **Audit**: Regularly audit your application's use of the API to detect any unauthorized access or unusual patterns.

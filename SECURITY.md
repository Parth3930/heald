# Security Policy

## Supported Versions

| Version | Supported |
|---|---|
| 0.1.x (beta) | ✅ Active |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Email security reports to: **sharmaparth.developer@gmail.com**

Include in your report:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

You will receive a response within 48 hours. If the issue is confirmed, we will:
1. Work on a fix
2. Release a patched version
3. Credit you in the release notes (unless you prefer to stay anonymous)

## Scope

Heald is a local CLI tool that reads and writes files on your machine. It does not make network requests, store data remotely, or handle authentication tokens. The primary security considerations are:

- **File path traversal** — untrusted `.heald/` content should not be able to write outside the project directory
- **YAML injection** — malformed frontmatter in OKF files should not cause arbitrary code execution

Thank you for helping keep Heald secure.

# Security Policy

## Overview

Security is a core concern for **Nozomi**.

Because this library performs **irreversible data deletion operations**,
bugs or unexpected behaviour could lead to data loss or security issues.
For this reason, security reports are taken seriously.

This document explains how to report vulnerabilities and which versions
of the project are supported.

------------------------------------------------------------------------

## Supported Versions

The following versions currently receive security support.

| Version | Status             | Security Support                      |
|---------|--------------------|---------------------------------------|
| 3.x     | Actively supported | Security fixes and improvements       |
| 2.x     | Passive support    | Dependency updates and critical fixes |
| 1.x     | End of life        | No longer supported                   |


Passive support versions may receive **critical security fixes or
dependency updates**, but no new features.

------------------------------------------------------------------------

## Reporting a Vulnerability

If you discover a potential security vulnerability, **please do not open
a public GitHub issue**.

Instead, report it privately using one of the following methods:

### Preferred method

Open a **GitHub Security Advisory**: https://github.com/Sans-Atout/Nozomi/security/advisories/new

### Alternative

Contact the maintainer privately.

When reporting a vulnerability, please include:

-   a clear description of the issue
-   steps to reproduce the problem
-   the potential impact
-   a proof of concept if available

This helps evaluate and address the issue quickly.

------------------------------------------------------------------------

## Disclosure Process

When a vulnerability is reported, the following process is typically
followed:

1.  The report is reviewed and validated.
2.  A fix is developed and tested.
3.  A security release is prepared if necessary.
4.  The vulnerability is publicly disclosed with the release notes.

Because this project is maintained during personal time, response times
may vary.

------------------------------------------------------------------------

## Storage and Filesystem Limitations

Nozomi performs **overwrite-based deletion at the filesystem level**.

Due to the behaviour of modern storage technologies, secure deletion
cannot be guaranteed in all environments.

Limitations may exist with:

-   SSD wear leveling
-   journaling filesystems
-   RAID controllers
-   filesystem snapshots
-   network filesystems

For highly sensitive environments, full disk encryption combined with
cryptographic key destruction may provide stronger guarantees.

------------------------------------------------------------------------

## Cryptographic Considerations

Nozomi does **not provide cryptographic data destruction guarantees**.

Random overwrite passes rely on randomness provided through the Rust
ecosystem.

The `secure_log` feature hashes identifiers to reduce information
exposure, but this hashing is **not intended as a cryptographic security
mechanism**.

------------------------------------------------------------------------

## Responsible Disclosure

If you discover a vulnerability, please allow time for the issue to be
fixed before publicly disclosing the details.

Responsible disclosure helps protect users of the library.

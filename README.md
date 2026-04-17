# Nozomi

[![Crates.io](https://img.shields.io/crates/v/nozomi)](https://crates.io/crates/nozomi)
[![Docs.rs](https://docs.rs/nozomi/badge.svg)](https://docs.rs/nozomi)
[![License](https://img.shields.io/crates/l/nozomi)](#license)
[![maintenance-status: actively-developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)](https://crates.io/crates/nozomi)

A Rust library for **secure file deletion**. Nozomi overwrites file contents with
industry-standard byte patterns before removing them from the filesystem, making
data recovery significantly harder on traditional storage media.

It implements the same sanitisation standards as the
[Eraser](https://eraser.heidi.ie) software for Windows.

> **SSD users — read the [important notice](#-important-notice-for-ssd-users)
> before integrating this library.**

---

## Quick start

```toml
[dependencies]
nozomi = "3.0.3"
```

```rust
use nozomi::{DeleteRequest, DeleteMethod, Method};

let report = DeleteRequest::builder()
    .path("/path/to/sensitive/file.txt")
    .method(DeleteMethod::BuiltIn(Method::Gutmann))
    .build()?
    .run()?;

println!("Deleted {:?} using {}", report.path, report.method);
```

---

## Supported sanitisation standards

| Variant                  | Standard            | Passes |
|--------------------------|---------------------|:------:|
| `Method::PseudoRandom`   | Random data         |   1    |
| `Method::HmgiS5`         | HMGI S5             |   2    |
| `Method::Afssi5020`      | AFSSI 5020          |   3    |
| `Method::Dod522022ME`    | DoD 5220.22-M (ME)  |   3    |
| `Method::RcmpTssitOpsII` | RCMP TSSIT OPS-II   |   7    |
| `Method::Dod522022MECE`  | DoD 5220.22-M (ECE) |   7    |
| `Method::Gutmann`        | Gutmann             |   35   |

See [ERASE_METHOD.md](ERASE_METHOD.md) for a detailed description of each pass schedule.

---

## Optional features

| Feature       | Description                                                                                             |
|---------------|---------------------------------------------------------------------------------------------------------|
| `dry-run`     | Simulate a deletion without writing anything to disk                                                    |
| `verify`      | Read back the last overwrite pass to confirm it was written correctly                                   |
| `analyze`     | Inspect the pass schedule of a method before running it                                                 |
| `log`         | Emit trace-level log entries via the [`log`](https://docs.rs/log) façade                                |
| `secure_log`  | Like `log`, but replaces file paths with their MD5 hash to avoid leaking names                          |
| `error-stack` | *(Deprecated — removed in 4.0.0)* Richer error context via [`error-stack`](https://docs.rs/error-stack) |

Enable features in `Cargo.toml`:

```toml
[dependencies]
nozomi = { version = "3.1.0", features = ["verify", "dry-run"] }
```

---

## ⚠ Important notice for SSD users

**Overwrite-based deletion methods are not reliable on solid-state drives.**

On a traditional hard disk drive (HDD), each logical block maps directly to a
fixed physical location on the platter. Overwriting a block is therefore
guaranteed to destroy the previous magnetic signal at that exact location.

SSDs work differently. The **Flash Translation Layer (FTL)** abstracts the
physical NAND cells behind a logical address space and applies **wear leveling**
— a technique that deliberately spreads writes across all available cells to
maximise drive longevity. When Nozomi asks the OS to overwrite a file, the FTL
is free to redirect each write to a *different* set of physical cells, leaving
the original data intact in unmapped or reserved sectors.

### What this means in practice

| Storage type        | Overwrite-based deletion                           | Reliable? |
|---------------------|----------------------------------------------------|:---------:|
| HDD (spinning disk) | Physical sector is rewritten in place              |   ✅ Yes   |
| SSD / NVMe          | FTL may redirect writes; original cells may remain |   ❌ No    |
| USB flash drive     | Same FTL / wear leveling constraints as SSD        |   ❌ No    |
| SD card             | Same FTL / wear leveling constraints as SSD        |   ❌ No    |

### Recommended alternatives for SSDs

- **ATA Secure Erase / NVMe Format** — a drive-level command that instructs the
  controller to cryptographically erase or reset all cells, including
  over-provisioned and reserved areas. Supported by most modern drives and
  exposed by tools such as `hdparm` (Linux) or `nvme-cli`.
- **Full-disk encryption from the start** — if the drive is encrypted before any
  sensitive data is written, destroying the encryption key renders all data
  unrecoverable, regardless of what the FTL has done with the physical cells.

Nozomi is most effective on **HDDs and RAM-backed filesystems** (e.g. `tmpfs`).
Using it on an SSD will still overwrite the logical blocks visible to the OS,
which may be sufficient for a low-threat model, but it cannot provide the same
guarantees as on spinning media.

---

## Support

### Supported versions

| Version | Status              | End of phase |
|---------|---------------------|--------------|
| 3.x     | Actively supported  | —            |
| 2.x     | Passively supported | 02 Jun 2029  |
| 1.x     | Yanked              | 02 Jun 2025  |

### Support lifecycle

**Active support (current major):** bugs are fixed, new features are added, and
dependencies are audited weekly with `cargo audit`.

**Passive support (previous major, 5 years):** dependencies are updated every
three months, `cargo audit` runs monthly. If a CVE requires a code change, a
new minor version is published.

**End-of-life process (1 year):** no dependency updates, no issue triage. This
phase gives downstream projects time to migrate before the version is yanked on
[crates.io](https://crates.io/crates/nozomi/versions).

---

## License

Licensed under either of:

- [MIT](LICENCE-MIT.md)
- [Apache License, Version 2.0](LICENCE-APACHE.md)

at your option.

---

[Changelog](CHANGELOG.md) · [Contributing](CONTRIBUTING.md) · [Security](SECURITY.md)

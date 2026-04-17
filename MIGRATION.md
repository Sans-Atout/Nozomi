# Migration Guide

This document covers breaking changes between major versions of **Nozomi**
and explains how to update your code.

---

## Migrating from 2.x / 3.0.x to 3.1.0

### The deletion API has changed

Before 3.1.0, deletions were triggered through static methods on the `Method`
enum or through the `SecureDelete` struct:

```rust
// 2.x style — SecureDelete static methods
use nozomi::SecureDelete;
SecureDelete::afssi_5020("path/to/file")?;

// 3.0.x style — Method::delete()
use nozomi::Method::Afssi5020;
Afssi5020::delete("path/to/file")?;
```

These APIs are **deprecated in 3.1.0** and will be **removed in 4.0.0**.

The new API uses a builder:

```rust
use nozomi::{DeleteRequest, DeleteMethod, Method};

let report = DeleteRequest::builder()
    .path("path/to/file")
    .method(DeleteMethod::BuiltIn(Method::Afssi5020))
    .build()?
    .run()?;
```

### Quick reference

| Before (≤ 3.0.x)                             | After (3.1.0+)                                                                                         |
|----------------------------------------------|--------------------------------------------------------------------------------------------------------|
| `Method::Gutmann::delete("path")`            | `DeleteRequest::builder().path("path").method(DeleteMethod::BuiltIn(Method::Gutmann)).build()?.run()?` |
| `SecureDelete::gutmann("path")`              | same as above                                                                                          |
| `Method::Gutmann::delete_with("path", sink)` | `.run_with(sink)?`                                                                                     |

### Receiving progress events

The old API had no event system. The new API supports an optional `EventSink`:

```rust,no_run
use nozomi::{DeleteRequest, DeleteMethod, Method, EventSink, DeleteEvent};

struct MyLogger;

impl EventSink for MyLogger {
    fn emit(&mut self, event: DeleteEvent) {
        println!("{:?}", event);
    }
}

DeleteRequest::builder()
    .path("path/to/file")
    .method(DeleteMethod::BuiltIn(Method::Gutmann))
    .build()?
    .run_with(&mut MyLogger)?;
# Ok::<(), nozomi::Error>(())
```

### The `error-stack` feature is deprecated

If your project enabled the `error-stack` feature, it still works in 3.1.0
but will be **removed in 4.0.0**. Remove it now to avoid a harder migration
later:

```toml
# Before
nozomi = { version = "3.1.0", features = ["error-stack"] }

# After
nozomi = { version = "3.1.0" }
```

Adjust your error handling to use the standard `nozomi::Error` type directly.

---

## Migrating from 3.x to 4.0.0  *(upcoming)*

> This section will be completed when v4.0.0 is released.
> The changes below reflect the current roadmap.

### Legacy API removed

`SecureDelete`, `Method::delete()`, and all associated types will be removed.
The builder-based `DeleteRequest` API becomes the only entry point.

If you followed the migration steps above for 3.1.0, no further changes will
be needed for this part of the migration.

### Error model redesigned

The flat `Error` enum will be replaced by a structured hierarchy:

```rust
// Planned (4.0.0)
pub enum Error {
    Planning(PlanningError),
    Execution(ExecutionError),
    Io(IoError),
}
```

Update your `match` arms accordingly when 4.0.0 is released.

### `error-stack` feature removed

The `error-stack` feature is removed entirely. Any code still using it must
be updated to the standard error type before upgrading to 4.0.0.

### Custom overwrite strategies

v4.0.0 will introduce an `OverwriteMethod` trait. If you were working around
the absence of custom methods in 3.x, this will be the supported extension
point.

---

## Support lifecycle

| Version | Status              | End of support |
|---------|---------------------|----------------|
| 3.x     | Actively supported  | —              |
| 2.x     | Passively supported | 02 Jun 2029    |
| 1.x     | Yanked              | 02 Jun 2025    |

See [README.md](README.md#support) for the full support policy.

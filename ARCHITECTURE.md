# Architecture

This document describes the internal architecture of **Nozomi v3.1.x**.

It is intended for contributors and maintainers who want to understand how the
library is structured and how secure deletion operations are executed.

---

## Principles

**Explicit configuration** — all operations are configured through a builder.
No implicit behavior, no global state.

**Stable public API** — breaking changes are reserved for major releases only.
The internal engine can be refactored freely without affecting the public API.

**Minimal default build** — no optional capability is enabled by default.
Every optional behavior is controlled through a Cargo feature.

**Event-driven observability** — the engine emits structured events during
execution. Events are informational only and never influence control flow.

---

## Module Map

```
src/
├── lib.rs                        # Crate root — re-exports public surface
├── methods.rs                    # Method enum (7 built-in sanitisation standards)
├── analyze.rs                    # PassKind, PassInfo, AnalysisReport  [feature: analyze]
│
├── error/
│   └── mod.rs                    # Error, Result, PlanningError, ExecutionError, IoError, FSProblem
│
├── api/
│   └── delete/
│       ├── mod.rs                # Module doc and re-exports
│       ├── builder.rs            # DeleteRequestBuilder — fluent configuration
│       ├── request.rs            # DeleteRequest, DeleteMethod, NoopSink
│       └── report.rs             # DeleteReport — result of a completed deletion
│
└── engine/                       # Private — not part of the public API
    ├── mod.rs                    # Engine entry point
    ├── events.rs                 # DeleteEvent enum, EventSink trait
    ├── planner.rs                # ExecutionPlan, execution_plan()
    ├── executor.rs               # run(), dry_run()
    ├── utils.rs                  # delete_file(), delete_dir(), emit_safe()
    ├── verify.rs                 # verify_last_pass()             [feature: verify]
    └── overwrite/
        ├── mod.rs                # Method dispatcher
        ├── common.rs             # prepare_overwrite() — shared buffer logic
        ├── afssi_5020.rs         # AFSSI 5020          (3 passes)
        ├── dod_522022_me.rs      # DoD 5220.22-M ME    (3 passes)
        ├── dod_522022_mece.rs    # DoD 5220.22-M ECE   (7 passes)
        ├── hmgi_s5.rs            # HMGI S5             (2 passes)
        ├── pseudo_random.rs      # PseudoRandom        (1 pass)
        ├── rcmp_tssit_ops_ii.rs  # RCMP TSSIT OPS-II   (7 passes)
        └── gutmann.rs            # Gutmann             (35 passes)
```

---

## Public API

### `DeleteRequestBuilder`  (`src/api/delete/builder.rs`)

Entry point for all deletion operations. Exposes a fluent builder that
collects the target path and the chosen overwrite method, then produces
a validated `DeleteRequest`.

```rust
DeleteRequest::builder()
    .path("/path/to/file")
    .method(DeleteMethod::BuiltIn(Method::Gutmann))
    .build()?
```

### `DeleteRequest`  (`src/api/delete/request.rs`)

Immutable, fully validated deletion request. Calling `.run()` hands it
to the execution engine. Calling `.run_with(sink)` additionally forwards
structured events to a caller-supplied `EventSink`.

### `DeleteReport`  (`src/api/delete/report.rs`)

Returned on successful completion. Carries the resolved path and the
method that was used.

### `DeleteMethod`  (`src/api/delete/request.rs`)

Selects the overwrite strategy. Currently, holds a single variant:

```rust
pub enum DeleteMethod {
    BuiltIn(Method),
}
```

Additional variants (user-defined strategies) are planned for v4.x via
the `OverwriteMethod` trait.

### `Error`  (`src/error/mod.rs`)

A structured hierarchy mirroring the phases of a deletion, instead of a
single flat enum:

```rust
pub enum Error {
    Planning(PlanningError),     // invalid input, before any destructive operation
    Execution(ExecutionError),   // failure during overwrite/verification passes
    Io(IoError),                 // raw filesystem operation failure
}
```

| Variant                                | Raised when                                                        |
|-----------------------------------------|---------------------------------------------------------------------|
| `PlanningError::MissingParameter`       | `DeleteRequestBuilder::build()` called without `path` or `method`   |
| `PlanningError::StringConversion`       | A path is not valid UTF-8                                          |
| `ExecutionError::OverwritePass`         | An overwrite pass failed for a given `Method` and pass number       |
| `ExecutionError::VerificationFailed`    | A post-overwrite read-back mismatch (`verify` feature)              |
| `IoError::Filesystem`                   | A filesystem syscall failed (open/read/write/rename/delete/…)       |
| `IoError::NoFileName`                   | A path has no filename component (e.g. bare root, ends in `..`)     |

Each sub-error implements `From<T> for Error`, so call sites can propagate
with `?` without manually wrapping in `Error::Planning`/`Execution`/`Io`.

### `Method`  (`src/methods.rs`)

Enumerates the seven built-in sanitisation standards.

| Variant          | Standard            | Passes |
|------------------|---------------------|:------:|
| `PseudoRandom`   | Random data         |   1    |
| `HmgiS5`         | HMGI S5             |   2    |
| `Afssi5020`      | AFSSI 5020          |   3    |
| `Dod522022ME`    | DoD 5220.22-M (ME)  |   3    |
| `RcmpTssitOpsII` | RCMP TSSIT OPS-II   |   7    |
| `Dod522022MECE`  | DoD 5220.22-M (ECE) |   7    |
| `Gutmann`        | Gutmann             |   35   |

---

## Execution Flow

```
DeleteRequest::run()
        │
        ▼
engine::executor::run()
        │
        ├─ 1. planner::execution_plan()
        │        └─ walks the filesystem tree
        │           returns ExecutionPlan { files: Vec<PathBuf>, directories: Vec<PathBuf> }
        │           (directories are ordered deepest-first for safe removal)
        │
        ├─ 2. For each file:
        │        ├─ emit DeletionStarted
        │        ├─ overwrite::overwrite_file()          ← dispatches to method module
        │        │       └─ N passes via prepare_overwrite()
        │        │            ├─ fill buffer (zero / pattern / random)
        │        │            ├─ write buffer to file
        │        │            ├─ fsync
        │        │            └─ emit EntryOverwritePass { pass, total_passes }
        │        ├─ [verify::verify_last_pass()]         [feature: verify]
        │        ├─ utils::delete_file()
        │        │       └─ progressive zero-rename strategy before unlink
        │        ├─ emit EntryDeleted
        │        └─ emit DeletionFinished
        │
        └─ 3. For each directory (deepest first):
                 ├─ utils::delete_dir()
                 ├─ emit EntryDeleted
                 └─ emit DeletionFinished
```

---

## Event System  (`src/engine/events.rs`)

The `EventSink` trait decouples the engine from any consumer-side logic:

```rust
pub trait EventSink {
    fn emit(&mut self, event: DeleteEvent);
}
```

Rules enforced by the engine:

- Events are emitted via `utils::emit_safe()`, which catches panics from
  sink implementations so that a buggy sink can never abort a deletion.
- Events are **informational only** — a sink cannot affect control flow.
- Errors are always returned through `Result`, never through events.

`NoopSink` (in `request.rs`) is the default sink used by `run()`.
Consumers who need events call `run_with(sink)` instead.

---

## Optional Features

| Feature       | Effect                                                                         |
|---------------|--------------------------------------------------------------------------------|
| `dry-run`     | Adds `dry_run()` / `dry_overwrite_file()` paths — emits events without I/O     |
| `verify`      | Adds `verify_last_pass()` — reads back the final pass to confirm correctness   |
| `analyze`     | Exposes `Method::analyze()` → `AnalysisReport` with the full pass schedule     |
| `log`         | Emits `trace!` entries via the `log` façade at key engine steps                |
| `secure_log`  | Like `log`, but replaces file paths with their MD5 hash                        |

---

## Evolution Towards v4.x

The following architectural changes are planned for v4.0.0 (see open issues):

- ~~**Remove the legacy `SecureDelete` API**~~ — done. `SecureDelete`,
  `Method::delete()`, and the `api/delete/legacy.rs` migration bridge have
  been removed. `DeleteRequestBuilder` is now the only entry point.
- ~~**Remove `error-stack` feature**~~ — done. The `error-stack` dependency
  and every `#[cfg(feature = "error-stack")]` duplicate implementation have
  been removed.
- ~~**Redesign the error model**~~ — done. The flat `Error` enum is now a
  structured hierarchy of `PlanningError`, `ExecutionError`, and `IoError`
  (see [`Error`](#error--srcerrormodrs) above).
- **`OverwriteMethod` trait** — replaces the hardcoded dispatcher with a
  trait-based extension point, enabling user-defined overwrite strategies.
- **Deletion policies** — high-level aliases (NIST 800-88, DoD) above
  the raw `Method` enum.
- **Filesystem safety detection** — emit a warning event when the target
  filesystem may interfere with secure deletion (SSD, copy-on-write, network).
- **Dangerous target safeguards** — reject critical paths (`/`, `/etc`, `/usr`)
  unless an explicit override is set.

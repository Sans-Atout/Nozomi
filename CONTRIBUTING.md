# Contributing to Nozomi

Thank you for considering contributing to **Nozomi**.

Nozomi is a Rust library for secure file and directory deletion. Because
the library performs **irreversible operations on user data**,
correctness, predictability, and security are critical.

This document explains how to contribute to the project.

------------------------------------------------------------------------

## Table of Contents

-   [Versioning Policy](#versioning-policy)
-   [Project Architecture (3.1.x series)](#project-architecture-31x-series)
-   [Adding a New Erase Method](#adding-a-new-erase-method)
-   [Other Contributions](#other-contributions)
-   [Running Tests](#running-tests)
-   [Maintainer Notes](#maintainer-note)

------------------------------------------------------------------------

## Versioning Policy

Nozomi follows **Semantic Versioning**.

Version format: `vX.Y.Z`

### Major version (`vX.0.0`)

Major versions introduce **breaking changes** to the public API.

Examples:

-   removal of deprecated APIs
-   architectural redesign
-   public API refactoring

Users upgrading to a new major version may need to modify their code.

------------------------------------------------------------------------

### Minor version (`v1.Y.0`)

Minor releases introduce **new functionality without breaking the public
API**.

Examples:

-   new erase methods
-   new optional features
-   internal improvements

Upgrading between minor versions should not require code changes.

------------------------------------------------------------------------

### Patch version (v1.1.Z)

Patch releases include:

-   dependency updates
-   bug fixes
-   internal improvements

Documentation or spelling corrections do **not necessarily require a
release**.

------------------------------------------------------------------------

## Project Architecture (3.1.x series)

Version **3.1** introduced several architectural improvements intended
to increase long‑term maintainability.

Deletion operations follow the structure below:

Public API → Builder → Internal Engine

The internal engine coordinates:

-   planning deletion operations
-   executing overwrite passes
-   filesystem interactions

The execution engine is intentionally **private** so internal
refactoring does not break the public API.

------------------------------------------------------------------------

### Feature Gating

Optional capabilities are controlled through Cargo features.

Default configuration:

    default = []

Contributors must ensure that:

-   the default build remains minimal
-   optional features are properly gated
-   feature combinations compile independently

------------------------------------------------------------------------

### Legacy Compatibility

Some legacy APIs remain available in the **3.x series** for
compatibility.

These APIs are **deprecated** and will be removed in **version 4.0.0**.

Contributors should avoid introducing new dependencies on legacy code.

------------------------------------------------------------------------

## Adding a New Erase Method

New erase methods are welcome but must follow a structured process.

Before implementing a new method:

-   Ensure the method is documented in a reliable source
-   Ensure it is not already implemented
-   Open an issue or discussion describing the proposal

Implementation steps:

1.  Create a new file in: `src/engine/overwrite.rs`

2.  Implement the overwrite logic.

3.  Register the method in: `src/methods.rs`

4.  Implement the `Display` trait for the method.

5.  Add unit tests.

------------------------------------------------------------------------

## Other Contributions

Not all contributions involve implementing new erase methods.

Other useful contributions include:

-   improving documentation
-   fixing bugs
-   adding examples
-   correcting spelling or grammar
-   improving tests

------------------------------------------------------------------------

## Running Tests

Before submitting a pull request, please run the test suite.
```shell
    cargo test
    cargo test --all-features
    cargo clippy --all-features
```

Optional faster test runner:
```shell
cargo nextest run
```
All tests should pass before submitting a pull request.

------------------------------------------------------------------------

## Maintainer Note

This project is maintained during personal time.

Suggestions, improvements, and constructive feedback are always welcome.

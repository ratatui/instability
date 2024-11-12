# Instability

Rust API stability attributes for the rest of us.

[![Crates.io](https://img.shields.io/crates/v/instability.svg)](https://crates.io/crates/instability)
[![Documentation](https://docs.rs/instability/badge.svg)][documentation]
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
![Minimum supported Rust version](https://img.shields.io/badge/rustc-1.63+-yellow.svg)
[![Build](https://github.com/ratatui-org/instability/workflows/Check/badge.svg)](https://github.com/ratatui-org/instability/actions)
[![Build](https://github.com/ratatui-org/instability/workflows/Test/badge.svg)](https://github.com/ratatui-org/instability/actions)

## Overview

This is a fork of the [Stability](https://crates.io/crates/stability) crate with the
[blessing](https://github.com/sagebind/stability/issues/12) of the original maintainer Stephen M.
Coakley ([@sagebind](https://github.com/sagebind)). This crate provides attribute macros for
specifying API stability of public API items of a crate. For a quick example:

```rust
/// This function does something really risky!
///
/// Don't use it yet!
#[instability::unstable(feature = "risky-function")]
pub fn risky_function() {
    unimplemented!()
}
```

Please check out the [documentation] for detailed usage.

## Installation

Install via Cargo by adding to your `Cargo.toml` file:

```toml
[dependencies]
instability = "0.3"
```

### Supported Rust versions

The current release is only guaranteed to work with the latest stable Rust compiler.

## License

This project's source code and documentation are licensed under the MIT license. See the [LICENSE](LICENSE.md) file for details.

[documentation]: https://docs.rs/instability

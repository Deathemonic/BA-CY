# Blue Archive - Cryptography

A library for handling **Blue Archive** Cryptography.

## Installation

### Rust

Add this to your `Cargo.toml`:

```toml
[dependencies]
bacy = { git = "https://github.com/Deathemonic/BA-CY" }
```

or run:

```sh
cargo add --git https://github.com/Deathemonic/BA-CY bacy
```

### Other languages

Build `bacy-ffi` from this repository to get either a compiled library
(`.so` / `.dll` / `.dylib`) exposing a plain C API, or UniFFI-generated
bindings for Kotlin, Swift, Python, and Ruby.

```sh
# Plain C API only (default, no UniFFI dependency)
cargo build --release -p bacy-ffi

# With UniFFI bindings enabled
cargo build --release -p bacy-ffi --features uniffi
```

See [`crates/bacy-ffi`](crates/bacy-ffi) for details on generating
bindings for a specific language.

---

<sub>**Copyright** - Blue Archive is a registered trademark of NAT GAMES Co., Ltd., NEXON Korea Corp., and Yostar, Inc.
This project is not affiliated with, endorsed by, or connected to NAT GAMES Co., Ltd., NEXON Korea Corp., NEXON GAMES
Co., Ltd., IODivision, Yostar, Inc., or any of their subsidiaries or affiliates. All game assets, content, and materials
are copyrighted by their respective owners and are used for informational and educational purposes only.</sub>
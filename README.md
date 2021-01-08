Push More: Instantly send notifications through Telegram
===

This is a Rust crate for the [Push More](https://pushmore.io) service to easily send notifications through Telegram.

[![Build Status](https://travis-ci.org/neosilky/pushmore-rust-client.svg?branch=master)](https://travis-ci.org/neosilky/pushmore-rust-client)
[![Docs Status](https://docs.rs/pushmore/badge.svg)](https://docs.rs/pushmore)
[![On crates.io](https://img.shields.io/crates/d/pushmore.svg)](https://crates.io/crates/pushmore)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

- [Documentation](https://docs.rs/pushmore)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pushmore = "0.1"
```

and this to your crate root:

```rust
extern crate pushmore;
use pushmore::PushMore;
```

and then you can use it as follows:

```rust
let client = PushMore::new(); // This will use the key stored in the PUSH_MORE_KEY environment variable.
// or
let client = PushMore::new_with_key("<pushmore key>".to_string());

client.send("hello!".to_string());
```

## TODO

* Add tests

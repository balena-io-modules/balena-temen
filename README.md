# balena temen

[![Build Status](https://travis-ci.org/balena-io-modules/balena-temen.svg?branch=master)](https://travis-ci.org/balena-io-modules/balena-temen)

A templating engine for JSON.

Provides facilities to:

* evaluate JSON
* evaluate JSON with custom evaluation keyword, functions and filters
* evaluate single expression (result is of any type)
* evaluate single logical expression (result is a boolean)
* retrieve [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree) of any expression / identifier

## Goal

`balena-temen` crate is one small piece of the [balena] configuration project. This project has
no public / open specification yet, but we're working on it and it will be public once finished.

## Supported platforms

Anyone should be able to use this crate:

* as a Rust crate
* as an NPM package (Node.js)
* in the browser (Web Assembly)

NPM package / browser (Web Assembly) parts are not done yet and are under the construction.

## Usage 

Add as a dependency to your `Cargo.toml`:

```
[dependencies]
balena-temen = "0.1"
```

Evaluate simple JSON:

```rust
use balena_temen::eval;
use serde_json::json;

let data = json!({
    "wifi": {
        "ssid": "Balena Ltd",
        "id": {
            "$$eval": "super.ssid | slugify"
        }
    }
});
let evaluated = json!({
    "wifi": {
        "ssid": "Balena Ltd",
        "id": "balena-ltd"
    }
});

assert_eq!(eval(data).unwrap(), evaluated);
```

You can find more information in the [API documentation].

[balena]: https://www.balena.io/
[API documentation]: https://docs.rs/balena-temen/latest/balena_temen/

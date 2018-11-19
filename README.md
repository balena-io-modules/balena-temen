# balena temen

[![Build Status](https://travis-ci.org/balena-io-modules/balena-temen.svg?branch=master)](https://travis-ci.org/balena-io-modules/balena-temen)
[![Current Release](https://img.shields.io/github/tag/balena-io-modules/balena-temen.svg?style=flat-square)](https://github.com/balena-io-modules/balena-temen/tags)
[![License](https://img.shields.io/github/license/balena-io-modules/balena-temen.svg?style=flat-square)](https://github.com/balena-io-modules/balena-temen/blob/master/LICENSE)
[![Issues](https://img.shields.io/github/issues/balena-io-modules/balena-temen.svg?style=flat-square)](https://github.com/balena-io-modules/balena-temen/issues)

A templating engine for JSON.

Provides facilities to:

* evaluate JSON
* evaluate JSON with custom evaluation keyword, functions and filters
* evaluate single expression (result is of any type)
* evaluate single logical expression (result is a boolean)
* retrieve [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree) of any expression / identifier

<div align="center">
  <sub>an open source :satellite: project by <a href="https://www.balena.io">balena.io</a></sub>
</div>

## Goal

`balena-temen` crate is one small piece of the [balena] configuration project. This project has
no public / open specification yet, but we're working on it and it will be public once finished.

## Supported platforms

Anyone should be able to use this crate:

* as a Rust crate
* as an NPM package (Node.js)
* in the browser (Web Assembly)

NPM package / browser (Web Assembly) parts are not done yet and are under the construction.

## Documentation

* [API documentation]
* [Expression language] documentation

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

## Support

If you're having any problem, please [raise an issue] on GitHub or [contact us], and the [balena.io] team
will be happy to help.

## License

WiFi Connect is free software, and may be redistributed under the terms specified in
the [license].

[balena.io]: https://www.balena.io/
[contact us]: https://forums.balena.io/
[raise an issue]: https://github.com/balena-io-modules/balena-temen/issues/new
[API documentation]: https://docs.rs/balena-temen/latest/balena_temen/
[license]: https://github.com/balena-io-modules/balena-temen/blob/master/LICENSE
[Expression language]: https://github.com/balena-io-modules/balena-temen/blob/master/docs/expression.md

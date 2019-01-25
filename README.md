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

`balena-temen` crate is one small piece of the [balena.io] configuration project. This project has
no public / open specification yet, but we're working on it and it will be public once finished.

## Supported platforms

This library is written in the Rust language and can be used:

* directly, as a [Rust crate]
* as an isomorphic [NPM package] (NodeJS & browser)

## Documentation

* [API documentation]
* [Expression language documentation]
* [Changelog]

## Breaking changes

### Version 0.2

* All function and filter identifiers were uppercased
  * An example `uuidv4()` -> `UUIDV4()`

### Version 0.1

`$$eval` keyword was renamed to `$$formula`. You can still use `$$eval` if you want
by instantiating your own [Engine](https://docs.rs/balena-temen/latest/balena_temen/struct.Engine.html)
with the [EngineBuilder](https://docs.rs/balena-temen/latest/balena_temen/struct.EngineBuilder.html) and
[custom eval keyword](https://docs.rs/balena-temen/latest/balena_temen/struct.EngineBuilder.html#method.eval_keyword)
 registration.

## Usage

### Rust

Add as a dependency to your `Cargo.toml`:

```toml
[dependencies]
balena-temen = "0.1"
```

Evaluate simple JSON:

```rust
use balena_temen::evaluate;
use serde_json::json;

let data = json!({
    "wifi": {
        "ssid": "Balena Ltd",
        "id": {
            "$$formula": "super.ssid | slugify"
        }
    }
});
let evaluated = json!({
    "wifi": {
        "ssid": "Balena Ltd",
        "id": "balena-ltd"
    }
});

assert_eq!(evaluate(data).unwrap(), evaluated);
```

### Javascript

Install via npm

```sh
npm install --save balena-temen
```

Evaluate simple JSON:

```js
const bt = require('balena-temen');

console.log(
    bt.evaluate({
        "ssid": "Some Cool SSID!",
        "id": {
            "$$formula": "super.ssid | slugify"
        }
    })
);
```

An example of using this module in nodeJS is available in the `examples/node` folder:

```bash
cd examples/node
npm install
npm start
```

An example of using this module in the browser is available in the `examples/browser` folder:

```bash
cd examples/browser
npm install
npm start
```

Open `localhost:8080` in your browser and you should see evaluated JSON in the browser console.

## Support

If you're having any problem, please [raise an issue] on GitHub or [contact us], and the [balena.io] team
will be happy to help.

## License

`balena-temen` is open source software, and may be redistributed under the terms specified in
the [license].

[balena.io]: https://www.balena.io/
[contact us]: https://forums.balena.io/
[raise an issue]: https://github.com/balena-io-modules/balena-temen/issues/new
[API documentation]: https://docs.rs/balena-temen/latest/balena_temen/
[license]: https://github.com/balena-io-modules/balena-temen/blob/master/LICENSE
[Expression language documentation]: https://github.com/balena-io-modules/balena-temen/blob/master/docs/expression.md
[Rust crate]: https://crates.io/crates/balena-temen
[NPM package]: https://www.npmjs.com/package/balena-temen
[Changelog]: https://github.com/balena-io-modules/balena-temen/blob/master/CHANGELOG.md

# sharks-wasm

[![CI](https://github.com/c0dearm/sharks-wasm/workflows/Rust/badge.svg?branch=master)](https://github.com/c0dearm/sharks-wasm/actions)
[![Codecov](https://codecov.io/gh/c0dearm/sharks-wasm/branch/master/graph/badge.svg)](https://codecov.io/gh/c0dearm/sharks-wasm)
[![License](https://camo.githubusercontent.com/47069b7e06b64b608c692a8a7f40bc6915cf629c/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f6c6963656e73652d417061636865322e302532464d49542d626c75652e737667)](https://github.com/c0dearm/sharks-wasm/blob/master/COPYRIGHT)

WebAssembly bindings for [Sharks](https://crates.io/crates/sharks)

## Usage

```javascript
const sharks_wasm = require('@c0dearm/sharks-wasm');

const sharks = sharks_wasm.SharksJS.new(3);
const shares = sharks.deal([1, 2, 3, 4], 255);
const secret = sharks.recover(shares);
console.log(secret);
```

## Building and testing

1. Install [wasm-pack](https://crates.io/crates/wasm-pack): `cargo install wasm-pack`
2. Build: `wasm-pack build`
3. Test: `wasm-pack test --node`

# Contributing

If you find a vulnerability, bug or would like a new feature, [open a new issue](https://github.com/c0dearm/sharks-wasm/issues/new).

To introduce your changes into the codebase, submit a Pull Request.

Many thanks!

# License

sharks-wasm is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.

# Browser WASM runtime in Rust

## Usage
```
$ yarn serve
```

## Running the browser
Running these examples requires the WebAssembly JS-BigInt integration, which is not supported by default on any browser.

This can be enabled on chromium by starting chromium as follows:
```
$ chromium-browser --js-flags="--experimental-wasm-bigint"
```

### Building the wasm modules
See the [add directory](add) for more info.
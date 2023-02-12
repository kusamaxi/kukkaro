<picture>
  <source srcset="https://raw.githubusercontent.com/kusamaxi/kukkaro/main/public/favicon.png" media="(prefers-color-scheme: dark)">
  <img style="width: 150px" src="https://raw.githubusercontent.com/kusamaxi/kukkaro/main/public/favicon.svg" alt="kukkaro logo">
</picture>

# kukkaro
Web extension wallet using FIDO2 authentication  
  
Kusama ecosystem lacks secure wallet extension that has fido2 authentication enabled. Nobody wants to secure their money with passwords and native hardware wallet implementations like ledger wallet are never going to be able to support all the 100 parachains. Parity signer is way too technical to keep as a purely offline wallet for normal users. We believe that Fido2 provides great solution here.  

We are looking to use purely rust for extension to keep it real.

## Running kukkaro

```bash
cargo leptos watch
```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
5. `npm install -g sass` - install `dart-sass` (should be optional in future

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing Your Project
```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.  
Tests are located in end2end/tests directory.

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
start-axum
site/
```
Set the following enviornment variables (updating for your project as needed):
```text
LEPTOS_OUTPUT_NAME="start-axum"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

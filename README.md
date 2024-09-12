<div align="center">

# Tauri Plugin rudderstack

[![Documentation](https://docs.rs/tauri-plugin-rudderstack/badge.svg)][documentation]
[![Crates.io](https://img.shields.io/crates/v/tauri-plugin-rudderstack.svg)](https://crates.io/crates/tauri-plugin-rudderstack)
[![License](https://img.shields.io/crates/l/tauri-plugin-rudderstack.svg)](https://github.com/elefant-ai/tauri-plugin-rudderstack/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/tauri-plugin-rudderstack.svg)](https://crates.io/crates/tauri-plugin-rudderstack)

</div>




## Features
- [x] Rudderstack Identify Event
- [x] Rudderstack Track Event
- [x] Rudderstack Page Event
- [x] Rudderstack Screen Event
- [x] Rudderstack Group Event
- [x] Rudderstack Alias Event
- [x] Auto generation of anonymous Id
- [x] Provides trait to use in tauri app
- [x] Provides URL watcher to track page events

## Install
There are three general methods of installation that we can recommend.

1. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)
2. Pull sources directly from Github using git tags / revision hashes (most secure)
3. Git submodule install this repo in your tauri project and then use file protocol to ingest the source (most secure, but inconvenient to use)

Install the Core plugin by adding the following to your `Cargo.toml` file:

Add the following to your `Cargo.toml` file
```toml
[dependencies]
tauri-plugin-rudderstack = "*"
# alternatively, you can point to the git repository
tauri-plugin-rudderstack = { git = "https://github.com/elefant-ai/tauri-plugin-rudderstack", branch = "main" }
```
Add the following to your package.json file
ou can install the JavaScript Guest bindings using your preferred JavaScript package manager:

> Note: Since most JavaScript package managers are unable to install packages from git monorepos we provide read-only mirrors of each plugin. This makes installation option 2 more ergonomic to use.

```sh
pnpm add tauri-plugin-rudderstack-api
# or
npm add tauri-plugin-rudderstack-api
# or
yarn add tauri-plugin-rudderstack-api

# alternatively with Git:
pnpm add https://github.com/elefant-ai/tauri-plugin-rudderstack#main
# or
npm add https://github.com/elefant-ai/tauri-plugin-rudderstack#main
# or
yarn add https://github.com/elefant-ai/tauri-plugin-rudderstack#main
```

## Usage
First you need to register the core plugin with Tauri:

`src-tauri/src/main.rs`

```rust
fn main() {
    let data_plane = "https://<Your URL>.dataplane.rudderstack.com";
    let key = "<Your Write Key>";
    let anonymous_id: Option<String> = None;

    tauri::Builder::default()
        .plugin(tauri_plugin_rudderstack::init(data_plane, key, anonymous_id))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings and rust trait:

# TODO Finish the documentation

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.



[repository]: https://github.com/elefant-ai/tauri-plugin-rudderstack
[documentation]: https://docs.rs/tauri-plugin-rudderstack/
[examples]: https://github.com/elefant-ai/tauri-plugin-rudderstack/tree/main/examples/tauri-app

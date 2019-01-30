# syntect_wasm

Attempt to run [syntect](https://github.com/trishume/syntect) in the browser. Work in progress.

## Progress

- [x] Initial commit! Environment setup via docker-compose, lib with simple highlighting function exported.
- [x] Get same-file building under wasm. Forked to `./same-file`, added `noop.rs` which just returns errors for all of the main methods, and used [[patch]](https://doc.rust-lang.org/cargo/reference/manifest.html#the-patch-section) to make cargo use it when building. This is necessary because of [this issue](https://github.com/BurntSushi/same-file/issues/42).
- [ ] Get rust-onig building under wasm. Currently crashing with not a whole lot of information.
- [ ] ???
- [ ] Syntect is running in the browser!

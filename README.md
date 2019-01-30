# syntect_wasm

Attempt to run [syntect](https://github.com/trishume/syntect) in the browser. Work in progress.

## Progress

- [x] Initial commit! Environment setup via docker-compose, lib with simple highlighting function exported.
- [x] Get same-file building under wasm. Forked to `./same-file`, added `noop.rs` which just returns errors for all of the main methods, and used [[patch]](https://doc.rust-lang.org/cargo/reference/manifest.html#the-patch-section) to make cargo use it when building. This is necessary because of [this issue](https://github.com/BurntSushi/same-file/issues/42).
- [x] Get rust-onig building under wasm. Forked to `./rust-onig`, commented lines 86-91 out because they would panic while building; the `CARGO_CFG_TARGET_FAMILY` env var wasn't set. Should probably fix this upstream since it's trivial.
- [ ] Figure out a way past the current blocker: everything compiles but rust-lld fails with `error: unknown file type: regexec.o`, which signals to me that I need to compile oniguruma to something lld can actually understand.
- [ ] ???
- [ ] Syntect is running in the browser!

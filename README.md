# syntect_wasm

Attempt to run [syntect](https://github.com/trishume/syntect) in the browser. Work in progress.

## Progress

- [x] Initial commit! Environment setup via docker-compose, lib with simple highlighting function exported.
- [x] Get same-file building under wasm. Forked to `./same-file`, added `noop.rs` which just returns errors for all of the main methods, and used [[patch]](https://doc.rust-lang.org/cargo/reference/manifest.html#the-patch-section) to make cargo use it when building. This is necessary because of [this issue](https://github.com/BurntSushi/same-file/issues/42).
- [x] Get rust-onig building under wasm. Forked to `./rust-onig`, commented lines 86-91 out because they would panic while building; the `CARGO_CFG_TARGET_FAMILY` env var wasn't set. Should probably fix this upstream since it's trivial.
- [ ] Wait until webassembly lands in llvm stable :sadface:
- [ ] ???
- [ ] Syntect is running in the browser!

## Resources

- [llvm-target-webassembly](http://llvm.org/svn/llvm-project/llvm/trunk/lib/Target/WebAssembly/README.txt)
- [wasm object format](https://github.com/WebAssembly/tool-conventions/blob/master/Linking.md)
- [wasm lld port](https://lld.llvm.org/WebAssembly.html)
- [emscripten with llbm webassembly backend](https://emscripten.org/docs/compiling/WebAssembly.html#llvm-webassembly-backend)

At this point everything in rust-land is compiling, but it fails at the link stage. I _don't think_ that rust-ld (rust's fork of lld?) can link the (unstable) object format, but even if it could, I can't compile to the object format without building llvm from source with `-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly` and aint nobody got time for that. But truthfully I am not very knowledgeable about all these things!

For future me, I think that if llvm with wasm is installed, you should be able to run the following and it _may_ work.

```
EMCC_WASM_BACKEND=1 CC=emcc CFLAGS='-s WASM_OBJECT_FILES=1' wasm-pack build
```

Also for future me, remember that rust also supports an emscripten target! But I honestly have no interest in just getting this to work whatever means necessary.

# fuzzball
FuzzBall - Fuzzy audio plugin

Simple VST plugin based on https://github.com/RustAudio/vst-rs/ examples.

## Build

### For debuging
```
cargo build
./macos_vst_bundler.sh FuzzBall target/debug/libfuzzball.dylib
```
### For release
```
cargo build --release
./macos_vst_bundler.sh FuzzBall target/release/libfuzzball.dylib
```

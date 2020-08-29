# Fuzz Ball
VST plugin with Fuzz effect written in Rust.
Fuzz effect can be controlled with 3 controllers.

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

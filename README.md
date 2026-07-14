# Rustris
A free, open, portable block game

## Stack
This project uses macroquad for rendering and is entirely written in Rust.
I plan to release browser, Windows and Linux executables. Android APK builds are
available through the maintained [cargo-quad-apk fork](https://github.com/hoklims/cargo-quad-apk):

```sh
rustup target add aarch64-linux-android x86_64-linux-android
cargo +1.95.0 install --git https://github.com/hoklims/cargo-quad-apk
cargo quad-apk build --release --locked
```

The APK is written under `target/android-artifacts/release/apk/`.

### About
I made this game because I wanted to get the design and feeling I had when playing on
my dad's computer at 4yo to show it to my daughter.

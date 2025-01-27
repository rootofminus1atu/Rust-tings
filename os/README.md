# TheOs

## Prereqs
nightly rust

## Building
```
cargo build --target x86_64-the_os.json
```
now possible to just use `cargo build` with the `.cargo/config.toml` config

## Building but with a bootloader
```
cargo bootimage
```

## Running on qemu
```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-the_os/debug/bootimage-os.bin
```
now also possible with just `cargo run`
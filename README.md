tryna make an OS in rust and shit yknow

## Building
This project requires a nightly version of Rust because it uses some unstable features. At least nightly _2020-07-15_ is required for building. You might need to run `rustup update nightly --force` to update to the latest nightly even if some comp such as `rustfmt` are missing it.

You can build the project by running:
```
cargo build
```
To create a bootable disk image from the compiled kernel, install the [`bootimage`] tool:
[`bootimage`]: https://github.com/rust-osdev/bootimage
```
cargo install bootimage
```

After installing, create bootable disk image by running
```
cargo bootimage
```
file issue if problem

## Running

You can run the disk image in [QEMU] through:

[QEMU]: https://www.qemu.org/

```
cargo run
```

[QEMU] and the [`bootimage`] tool need to be installed for this.

You can also write the image to an USB stick for booting it on a real machine. On Linux, the command for this is:

```
dd if=target/x86_64-blog_os/debug/bootimage-blog_os.bin of=/dev/sdX && sync
```

Where `sdX` is the device name of your USB stick. **Be careful** to choose the correct device name, because everything on that device is overwritten.

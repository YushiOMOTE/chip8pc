chip8pc
=========================

Bare-metal chip8 for BIOS PCs.

### Prerequisites

* `rustup component add llvm-tools-preview`
* `cargo install cargo-binutils`
* [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild)

### Try

* Get [bootloader](https://github.com/rust-osdev/bootloader/)

    ```
    $ git submodule update --init
    ```

* Build/run on QEMU

    ```
    $ ./build.sh
    ```

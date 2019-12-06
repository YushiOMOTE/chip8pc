#!/bin/sh

set -xe

kernel=$(pwd)
target=$(pwd)/bootloader/target/x86_64-bootloader/release

cargo xbuild --release

pushd bootloader

export KERNEL=$kernel/target/x86_64-chip8pc/release/chip8pc
export KERNEL_MANIFEST=$kernel/Cargo.toml

cargo xbuild --release --features binary,vga_320x200
cargo objcopy -- --strip-all -I elf64-x86-64 -O binary --binary-architecture=i386:x86-64 \
      $target/bootloader $target/bootloader.bin
qemu-system-x86_64 -drive format=raw,file=$target/bootloader.bin

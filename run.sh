#!/bin/bash
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-zy_os/debug/bootimage-ZY_os.bin -display curses

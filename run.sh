#!/bin/sh
qemu-system-x86_64 -drive format=raw,file=target/x86_64-mark_os/debug/bootimage-mark_os.bin

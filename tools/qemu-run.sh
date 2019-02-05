#!/bin/sh

qemu-system-x86_64 -drive format=raw,file=target/x86_64-posos/debug/bootimage-posos.bin -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none

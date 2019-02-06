#!/bin/sh

extra_run_arg=""
extra_display_arg=""

if [ ! -z "$1" ]; then
	extra_run_arg="--bin $1"
	extra_display_arg="-display none"
fi

# either run the default binary or another if its name is passed to this
bootimage run $extra_run_arg -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -enable-kvm #$extra_display_arg

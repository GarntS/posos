// file:	lib.rs
// author:	garnt
// date:	2/4/2019
// desc:	the posos library..?

// don't link the rust stl.
#![cfg_attr(not(test), no_std)]
// allow use of x86-interrupt foreign call
#![feature(abi_x86_interrupt)]

pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

extern crate bit_field;

// exit_qemu() does exactly what you think it does
// qemu exposes this oddball debug-exit port if you ask it nicely.
pub unsafe fn exit_qemu() {
	use x86_64::instructions::port::Port;

	let mut port = Port::<u32>::new(0xf4);
	port.write(0);
}

// file:	lib.rs
// author:	garnt
// date:	2/4/2019
// desc:	the posos library..?

// don't link the rust stl.
#![cfg_attr(not(test), no_std)]
// allow use of x86-interrupt foreign call
#![feature(abi_x86_interrupt)]
// allow intrinsics for shit like unreachable()
#![feature(core_intrinsics)]
// allow inline assembly to make rust do what i want it to sometimes
#![feature(asm)]
// allow naked functions for some true fuckery
#![feature(naked_functions)]


pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

// import bitflags and bit_field
// we have to do this shenanigans so we can import bitflags' macros
#[macro_use]
extern crate bitflags;
extern crate bit_field;

// exit_qemu() does exactly what you think it does
// qemu exposes this oddball debug-exit port if you ask it nicely.
pub unsafe fn exit_qemu() {
	use x86_64::instructions::port::Port;

	let mut port = Port::<u32>::new(0xf4);
	port.write(0);
}

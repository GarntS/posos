// file:	main.rs
// author:	garnt
// date:	1/30/2019
// desc:	Entry point for posos kernel

// disables the rust stl. required to run on bare metal without the stl.
// disabling the stl breaks main() so we need to macro that off as well.
#![no_std]
#![no_main]

// includes
use core::panic::PanicInfo;
// mods
mod vga_buffer;

// this function is called when rust panics.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	// loop infinitely.
	loop {}
}

// make a linux-friendly _start function. no_mangle prevents the compiler
// from screwing this up by naming it something stupid.
#[no_mangle]
pub extern "C" fn _start() -> ! {
	// Oh boy let's test the driver
	vga_buffer::print_test();

	// Hold state indefinitely
	loop {}
}

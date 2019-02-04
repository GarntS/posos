// file:	main.rs
// author:	garnt
// date:	1/30/2019
// desc:	Entry point for posos kernel

// disables the rust stl. required to run on bare metal.
// disabling the stl breaks main() so we need to macro that off as well.
// both of these things are enabled for testing
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
// allow unused imports during testing
#![cfg_attr(test, allow(unused_imports))]

// includes
use core::panic::PanicInfo;
// mods
mod serial;
mod vga_buffer;

// this function is called when rust panics. (disabled when testing)
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	// tell me why you panicked, then loop infinitely
	println!("{}", info);
	loop {}
}

// make a linux-friendly _start function. no_mangle prevents the compiler
// from screwing this up by naming it something stupid. (disabled when
// testing)
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	// Test write
	println!("GARBAGE! {}", 420.69);

	// Test panic
	panic!("Whoops, i panicked");

	// Hold state indefinitely
	//loop {}
}

// file:	main.rs
// author:	garnt
// date:	1/30/2019
// desc:	Entry point for posos kernel

// disables the rust stl. required to run on bare metal.
// disabling the stl breaks main() so we need to macro that off as well.
// also, allow unused imports during testing
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

// includes
use core::panic::PanicInfo;
use posos::{exit_qemu,println};

// this function is called when rust panics. tells you why and then loops.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}

// make a bare metal-friendly _start function. no_mangle muzzles the compiler
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("GARBAGE! {}", 420.69);

	// initialize our interrupts
	posos::interrupts::init();

	println!("It's all good my dude -cory");
	unsafe { exit_qemu(); }
	// Hold state indefinitely
	loop {}
}

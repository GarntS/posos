// file:	test-basic-boot.rs
// author:	garnt
// date:	2/4/2019
// desc:	Integration test that tests basic booting functionality

// disables the rust stl. required to run on bare metal.
// disabling the stl breaks main() so we need to macro that off as well.
// also, allow unused imports during testing
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

// includes
use core::panic::PanicInfo;
use posos::{exit_qemu, serial_println};

// this function is called when rust panics. tells you why and then exits.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	serial_println!("boot failed.");
	serial_println!("{}", info);

	unsafe { exit_qemu(); }
	loop {}
}

// make a bare metal-friendly _start function. no_mangle muzzles the compiler
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	serial_println!("booted ok!");

	unsafe { exit_qemu(); }
	loop {}
}

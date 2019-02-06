// file:	test-exception-pagefault.rs
// author:	garnt
// date:	2/4/2019
// desc:	Integration test that tests page faults

// disables the rust stl. required to run on bare metal.
// disabling the stl breaks main() so we need to macro that off as well.
// also, allow unused imports during testing
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

// allow asm for the purpose of making a divide by zero
#![feature(asm)]

// includes
use core::panic::PanicInfo;
use posos::{exit_qemu, serial_println};

// this function is called when rust panics. tells you why and then exits.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	serial_println!("test failed.");
	serial_println!("{}", info);

	unsafe { exit_qemu(); }
	loop {}
}

// make a bare metal-friendly _start function. no_mangle muzzles the compiler
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	// initialize the idt
	posos::interrupts::init();

	// cafebabe is free, they shall never be changed by your puny bad rust
	unsafe { *(0xcafebabe as *mut u64) = 12 };

	serial_println!("ok");

	unsafe { exit_qemu(); }
	loop {}
}

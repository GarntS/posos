// file:	mod.src
// author:	garnt
// date:	2/4/2019
// desc:	Generic interrupts module which wraps multiple architectures

// includes
use lazy_static::lazy_static;
use crate::println;

// declare the submodules
mod x86;

/// divide_by_zero_handler() is exactly what you think it is
extern "C" fn divide_by_zero_handler() -> ! {
	println!("EXCEPTION: DIVIDE BY ZERO");
	loop {}
}

// create a static instance of Idt to act as the global
lazy_static! {
	static ref IDT: x86::idt::Idt = {
		let mut idt = x86::idt::Idt::new();
		idt.set_handler(0, divide_by_zero_handler);
		idt
	};
}

/// init() TODO(garnt): document this properly
pub fn init_idt() {
	IDT.load();
}

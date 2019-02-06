// file:	mod.src
// author:	garnt
// date:	2/4/2019
// desc:	Generic interrupts module which wraps multiple architectures

// declare the submodules
mod x86;

/// init() initializes the interrupt interface
pub fn init() {
	x86::init_idt();
}

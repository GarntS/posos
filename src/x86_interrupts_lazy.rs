// file:	x86_interrupts.rs
// author:	garnt
// date:	2/4/2019
// desc:	Handlers for various x86 interrupts.

// includes
use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};

// lazy_static instance containing the InterruptDescriptorTable object
lazy_static! {
	static ref IDT: InterruptDescriptorTable =  {
			let mut idt = InterruptDescriptorTable::new();
			idt.breakpoint.set_handler_fn(breakpoint_handler);
			idt
	};
}

/// init_idt() initializes the IDT object with our handlers
pub fn init_idt() {
	IDT.load();
}

/// breakpoint_handler() is the handler for breakpoints
extern "x86-interrupt" fn breakpoint_handler(
	stack_frame: &mut ExceptionStackFrame)
{
	println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

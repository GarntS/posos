// file:	x86.rs
// author:	garnt
// date:	2/4/2019
// desc:	x86-arch interrupt handler implementation

// includes
use crate::println;
use lazy_static::lazy_static;

// struct to represent the exception stack frame
#[derive(Debug)]
#[repr(C)]
struct ExceptionStackFrame {
	instruction_pointer: u64,
	code_segment: u64,
	cpu_flags: u64,
	stack_pointer: u64,
	stack_segment: u64,
}

/// handler! wraps the main handler in a function and grabs the stack frame.
/// we denote it as a naked function to prevent a prologue from being added and
/// fucking up the inline assembly from grabbing the stack frame
macro_rules! handler {
	($name: ident) => {{
		#[naked]
		extern "C" fn wrapper() -> ! {
			unsafe {
				// asm. load the stack pointer into rdi, then call the handler
				asm!("mov rdi, rsp
						sub rsp, 8 // align the stack pointer to a 16-byte bound
						call $0"
						:: "i"($name as extern "C" fn(_) -> !)
						: "rdi" : "intel");
				// tell the rust compiler that this, in fact, an unreachable
				// bit of code. otherwise, the compiler would be too stupid 
				// to read the inline assembly, and know that it diverges
				::core::intrinsics::unreachable();
			}
		}
		wrapper
	}}
}

/// handler_with_error_code! does the same thing as handler, but is adjusted
/// to take an additional error code argument
macro_rules! handler_with_error_code {
	($name: ident) => {{
		#[naked]
		extern "C" fn wrapper() -> ! {
			unsafe {
				// asm. load the stack pointer into rdi, then call the handler
				asm!("pop rsi // pop the error code into rsi
						mov rdi, rsp
						sub rsp, 8 // align the stack pointer to a 16-byte bound
						call $0"
						:: "i"($name as extern "C" fn(
							&ExceptionStackFrame, u64) -> !)
						: "rdi","rsi" : "intel");
				// tell the rust compiler that this, in fact, an unreachable
				// bit of code. otherwise, the compiler would be too stupid 
				// to read the inline assembly, and know that it diverges
				::core::intrinsics::unreachable();
			}
		}
		wrapper
	}}
}

/// divide_by_zero_handler() is exactly what you think it is.
extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) -> ! {
	// print out the stack frame with this
	println!("\nEXCEPTION! Divide by Zero\n{:#?}", &*stack_frame);
	loop {}
}

/// invalid_opcode_handler() is exactly what you think it is.
extern "C" fn invalid_opcode_handler(stack_frame: &ExceptionStackFrame) -> ! {
	let stack_frame = &*stack_frame;
	println!("\nEXCEPTION! Invalid Opcode at {:#x}\n{:#?})",
				stack_frame.instruction_pointer, stack_frame);
	loop{}
}

// struct with constants used for translating page fault's error codes
bitflags! {
	struct PageFaultErrorCode: u64 {
		const PROTECTION_VALIDATION=1 << 0;
		const CAUSED_BY_WRITE = 1 << 1;
		const USER_MODE = 1 << 2;
		const MALFORMMED_TABLE = 1 << 3;
		const INSTRUCTION_FETCH = 1 << 4;
	}
}

/// page_fault_handler() is exactly what you think it is.
extern "C" fn page_fault_handler(stack_frame: &ExceptionStackFrame,
									error_code: u64) -> !
{
	use x86_64::registers::control;
	println!("\nEXCEPTION! Page Fault while accessing {:#x}\
				\nerror code: {:?}\n{:#?}",
				unsafe { control::Cr2::read().as_u64() },
				PageFaultErrorCode::from_bits(error_code).unwrap(),
				&*stack_frame);
	loop {}
}

// create a static instance of Idt to act as the global
lazy_static! {
	static ref IDT: idt::Idt = {
		let mut idt = idt::Idt::new();
		idt.set_handler(0, handler!(divide_by_zero_handler));
		idt.set_handler(6, handler!(invalid_opcode_handler));
		idt.set_handler(14, handler_with_error_code!(page_fault_handler));
		idt
	};
}

/// init_idt() initializes the global instance of an x86 idt
pub fn init_idt() {
	IDT.load();
}

// rust module implementing x86 idt(interrupt descriptor tables)
pub mod idt {
	use bit_field::BitField;
	use x86_64::instructions::segmentation;
	use x86_64::PrivilegeLevel;
	use x86_64::structures::gdt::SegmentSelector;

	// define HandlerFunc as c function that diverges
	pub type HandlerFunc = extern "C" fn() -> !;

	// struct of Idt, which is an array of 16 entries
	pub struct Idt([Entry; 16]);

	// implementation for Idt
	impl Idt {
		/// new() is the constructor for Idt
		pub fn new() -> Idt {
			Idt([Entry::missing(); 16])
		}

		/// set_handler() adds new handlers to the idt
		pub fn set_handler(&mut self, entry: u8, handler: HandlerFunc)
			-> &mut EntryOptions
		{
			self.0[entry as usize] = Entry::new(segmentation::cs(), handler);
			&mut self.0[entry as usize].options
		}

		/// load() actually loads the idt into memory. note that this has
		/// a static lifetime so that it is impossible for this object to
		/// go out of memory. this means we can ignore some future unsafety.
		pub fn load(&'static self) {
			use x86_64::instructions::tables::{DescriptorTablePointer, lidt};
			use core::mem::size_of;

			// construct a pointer to the idt represented by this object
			let ptr = DescriptorTablePointer {
				base: self as *const _ as u64,
				limit: (size_of::<Self>() -1) as u16,
			};

			// lidt means load idt, do that at the position of the idt pointer
			unsafe { lidt(&ptr) };
		}
	}

	// struct representing an idt entry
	#[derive(Debug, Clone, Copy)]
	#[repr(C, packed)]
	pub struct Entry {
		pointer_low: u16,
		gdt_selector: SegmentSelector,
		options: EntryOptions,
		pointer_middle: u16,
		pointer_high: u32,
		reserved: u32,
	}

	// implementation for Entry
	impl Entry {
		/// new() is the default constructor for Entry
		fn new(gdt_selector: SegmentSelector, handler: HandlerFunc) -> Self {
			let pointer = handler as u64;
			Entry {
				gdt_selector: gdt_selector,
				pointer_low: pointer as u16,
				pointer_middle: (pointer >> 16) as u16,
				pointer_high: (pointer >> 32) as u32,
				options: EntryOptions::new(),
				reserved: 0,
			}
		}

		/// missing() constructs an Entry for interrupts that are missing
		fn missing() -> Self {
			Entry {
				gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
				pointer_low: 0,
				pointer_middle: 0,
				pointer_high: 0,
				options: EntryOptions::minimal(),
				reserved: 0
			}
		}
	}

	// EntryOptions contains some options for 
	#[derive(Debug, Clone, Copy)]
	pub struct EntryOptions(u16);

	// implementation for EntryOptions
	impl EntryOptions {
		/// minimal() constructs a barebones instance of EntryOptions
		fn minimal() -> Self {
			let mut options = 0;
			options.set_bits(9..12, 0b111);	// set all the 'must-be-one' bits
			EntryOptions(options)
		}

		/// new() constructs a full instance of EntryOptions
		fn new() -> Self {
			let mut options = Self::minimal();
			options.set_present(true).disable_interrupts(true);
			options
		}

		/// set_present() is a setter for the present bit
		pub fn set_present(&mut self, present: bool) -> &mut Self {
			self.0.set_bit(15, present);
			self
		}

		/// disable_interrupts() TODO(garnt): label
		pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
			self.0.set_bit(8, !disable);
			self
		}

		/// set_privilege_level() TODO(garnt): label
		pub fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
			self.0.set_bits(13..15, dpl);
			self
		}

		/// set_stack_index() TODO(garnt): label
		pub fn set_stack_index(&mut self, index: u16) -> &mut Self {
			self.0.set_bits(0..3, index);
			self
		}
	}
}

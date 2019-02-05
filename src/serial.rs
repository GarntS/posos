// file:	serial.rs
// author:	garnt
// date:	2/4/2019
// desc:	UART driver to manage sending information over serial.

// includes
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

// we have to lazy_static this because rust tags
lazy_static! {
	// ref to the serial port at the first serial port address, which is 0x3F8
	pub static ref SERIAL_1: Mutex<SerialPort> = {
		let mut serial_port = SerialPort::new(0x3F8);
		serial_port.init();
		Mutex::new(serial_port)
	};
}

// _print is an internal  implementation of writing to the serial port
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
	use core::fmt::Write;
	SERIAL_1.lock().write_fmt(args).expect("Printing to Serial FAILED!");
}

/// Prints to the serial interface
#[macro_export]
macro_rules! serial_print {
	($($arg:tt)*) => {
		$crate::serial::_print(format_args!($($arg)*));
	};
}

/// Prints to the serial interface, adding a newline
#[macro_export]
macro_rules! serial_println {
	() => ($crate::serial_print!("\n"));
	($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
		concat!($fmt, "\n"), $($arg)*));
}

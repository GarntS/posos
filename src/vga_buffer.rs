// file:	vga_buffer.rs
// author:	garnt
// date:	10/30/2019
// desc:	Soopah-simplistic VGA text mode driver.

// includes.
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

// Some constants defining the buffer dimensions
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// allow dead_code allows enum entries that aren't always in-use
// repr(u8) byte-aligns each VGAColor to a uint8, as u4 isn't a type
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}

// ColorCode contains a full foreground and background color-code pair
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
	fn new(foreground: Color, background: Color) -> ColorCode {
		ColorCode((background as u8) << 4 | (foreground as u8))
	}
}

// ScreenChar contains the character to be displayed and its ColorCode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
	ascii_character: u8,
	color_code: ColorCode,
}

// Buffer contains a BUFFER_WIDTH x BUFFER_HEIGHT 2D array of ScreenChar's
struct Buffer {
	// note: we define this as volatile to prevent rust optimization
	// breaking our nice clean writes into memory
	chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// Writer is a public struct that manages buffer, which lasts the lifetime
// of the program. The Buffer is "probably stored in .data"
pub struct Writer {
	column_position: usize,
	color_code: ColorCode,
	buffer: &'static mut Buffer,
}

// Public static instance of Writer to be used for writing.
// we make it a lazy_static because something something rust
// we wrap it in a mutex so that it's "interior mutable"
lazy_static! {
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
		column_position: 0,
		color_code: ColorCode::new(Color::LightRed, Color::Black),
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
	});
}

impl Writer {
	// write_byte writes a byte to the buffer
	pub fn write_byte (&mut self, byte: u8) {
		match byte {
			// if we recieve a carraige return
			b'\n' => self.new_line(),
			//
			byte => {
				// if we're at or over the width, call new_line()
				if self.column_position >= BUFFER_WIDTH {
					self.new_line();
				}

				// Fetch the coords we're gonna write at and the color
				let row = BUFFER_HEIGHT - 1;
				let col = self.column_position;
				let color_code = self.color_code;

				// Actually write it to the buffer, we're using .write(
				// instead of just = because Volatile<>
				self.buffer.chars[row][col].write(ScreenChar {
					ascii_character: byte,
					color_code,
				});
				self.column_position += 1;
			}
		}
	}

	// write_string writes a string to the buffer
	pub fn write_string(&mut self, s: &str) {
		// Loop through the bytes in the string
		for byte in s.bytes() {
			match byte {
				// printable ASCII byte or a newline
				0x20...0x7e | b'\n' => self.write_byte(byte),
				// not part of the printable ASCII range
				_ => self.write_byte(0xfe),
			}
		}
	}

	// new_line moves to the next line
	fn new_line(&mut self) {
		// iterate through the buffer, lines 2-n
		for row in 1..BUFFER_HEIGHT {
			for col in 0..BUFFER_WIDTH {
				// move every character up a line
				let character = self.buffer.chars[row][col].read();
				self.buffer.chars[row-1][col].write(character);
			}
		}
		// clear out the last row and set curor pos to the start of it
		self.clear_row(BUFFER_HEIGHT - 1);
		self.column_position = 0;
	}

	// clear_row clears out a row
	fn clear_row(&mut self, row: usize) {
		// blank character constant
		let blank = ScreenChar {
			ascii_character: b' ',
			color_code: self.color_code,
		};
		// loop through the row and shove a blank character everywhere
		for col in 0..BUFFER_WIDTH {
			self.buffer.chars[row][col].write(blank);
		}
	}
}

// Implementation of rust's fmt::write for the Writer class
impl fmt::Write for Writer {
	// write_str basically just wraps Writer::write_string()
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

// test_writer is a module containing entirely unit tests for Writer.
#[cfg(test)]
mod test_writer {
	use super::*;
	use array_init::array_init;

	// construct_writer() tests our ability to construct a Writer instance
	fn construct_writer() -> Writer {
		use std::boxed::Box;

		let buffer = construct_buffer();
		Writer {
			column_position: 0,
			color_code: ColorCode::new(Color::Cyan, Color::Green),
			buffer: Box::leak(Box::new(buffer)),
		}
	}

	// construct_buffer() tests our ability to constuct a Buffer instance
	fn construct_buffer() -> Buffer {
		Buffer {
			// we're using this array_init shenanigans because of rust-iness
			// and something with the Copy attribute and array constructors
			// weird-ass syntax is a closure, which is ~ an inline-function
			chars: array_init(|_| array_init(|_| Volatile::new(empty_char()))),
		}
	}

	// empty_char() is a helper that constructs an empty ScreenChar
	fn empty_char() -> ScreenChar {
		ScreenChar {
			ascii_character: b' ',
			color_code: ColorCode::new(Color::Cyan, Color::Brown),
		}
	}

	// write_byte() tests writing bytes to the vga buffer via Writer
	#[test]
	fn write_byte() {
		let mut writer = construct_writer();
		writer.write_byte(b'A');
		writer.write_byte(b'B');

		// iterate through each character in the buffer
		for (i, row) in writer.buffer.chars.iter().enumerate() {
			for (j, screen_char) in row.iter().enumerate() {
				let screen_char = screen_char.read();
				// check that the characters got printed properly
				if i == BUFFER_HEIGHT-1 && j == 0 {
					assert_eq!(screen_char.ascii_character, b'A');
					assert_eq!(screen_char.color_code, writer.color_code);
				} else if i == BUFFER_HEIGHT-1 && j==1 {
					assert_eq!(screen_char.ascii_character, b'B');
					assert_eq!(screen_char.color_code, writer.color_code);
				} else {
					assert_eq!(screen_char, empty_char());
				}
			}
		}
	}

	// write_formatted() tests writing formatted strings to the vga buffer
	#[test]
	fn write_formatted() {
		use core::fmt::Write;

		let mut writer = construct_writer();
		writeln!(&mut writer, "A").unwrap();
		writeln!(&mut writer, "B{}", "C").unwrap();

		// iterate through each character in the buffer
		for (i, row) in writer.buffer.chars.iter().enumerate() {
			for (j, screen_char) in row.iter().enumerate() {
				let screen_char = screen_char.read();
				// check that the strings got printed properly
				if i == BUFFER_HEIGHT-3 && j == 0 {
					assert_eq!(screen_char.ascii_character, b'A');
					assert_eq!(screen_char.color_code, writer.color_code);
				} else if i == BUFFER_HEIGHT-2 && j == 0 {
					assert_eq!(screen_char.ascii_character, b'B');
					assert_eq!(screen_char.color_code, writer.color_code);
				} else if i == BUFFER_HEIGHT-2 && j == 1 {
					assert_eq!(screen_char.ascii_character, b'C');
					assert_eq!(screen_char.color_code, writer.color_code);
				} else if i >= BUFFER_HEIGHT-2 {
					assert_eq!(screen_char.ascii_character, b' ');
					assert_eq!(screen_char.color_code, writer.color_code);
				} else {
					assert_eq!(screen_char, empty_char());
				}
			}
		}
	}
}

// print macro
#[macro_export]
macro_rules! print {
	// TODO(garnt): figure out how the fuck this syntax works
	($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

// println macro
#[macro_export]
macro_rules! println {
	// TODO(garnt): figure out how the fuck this syntax works
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// _print() basically just implements the actual functionality for the
// print! and println! macros
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	WRITER.lock().write_fmt(args).unwrap();
}

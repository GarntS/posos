// file:	vga_buffer.rs
// author:	garnt
// date:	10/30/2019
// desc:	Soopah-simplistic VGA text mode driver.

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
	chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// Writer is a public struct that manages buffer, which lasts the lifetime
// of the program. The Buffer is "probably stored in .data"
pub struct Writer {
	column_position: usize,
	color_code: ColorCode,
	buffer: &'static mut Buffer,
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

				// Actually write it to the buffer, then slide columns
				self.buffer.chars[row][col] = ScreenChar {
					ascii_character: byte,
					color_code,
				};
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
	pub fn new_line(&mut self) {/* TODO */}
}

// print_test() tests the driver
pub fn print_test() {
	let mut writer = Writer {
		column_position: 0,
		color_code: ColorCode::new(Color::LightRed, Color::Black),
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
	};

	writer.write_byte(b'H');
	writer.write_string("ello ");
	writer.write_string("Wörld!");
}

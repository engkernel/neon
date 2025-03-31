const SCREEN_HEIGHT: usize = 25;
const SCREEN_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	Black = 0,
	White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
	fn new(foreground: Color, background: Color) -> ColorCode {
	   ColorCode((foreground as u8) << 4 | (background as u8));
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct VGABuffer {
	chars: [[ScreenChar; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

pub struct Log {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,	
}

impl Log {
    pub fn write_byte(&mut self, byte: u8) {
	match byte {
	   b'\n' => self.new_line(),
	   byte => {
	      if self.column_position >= SCREEN_WIDTH {
		  self.new_line();
	      }
 	   }
	   
	   let row = SCREEN_HEIGHT - 1;
  	   let col = self.column_position;

	   let color_code = self.color_code;
 	   self.buffer.chars[row][col] = ScreenChar {
	       ascii_character: byte,
	       color_code,
	   }
        }
    }

    fn new_line(&mut self) {}

    pub fn write_string(&mut self, s: &str) {
	for byte in s.bytes() {
	    match byte {
		0x20..=0x7e | b'\n' => self.write_byte(byte),
		_ => self.write_byte(0xfe),
	    }
	}
    } 
}

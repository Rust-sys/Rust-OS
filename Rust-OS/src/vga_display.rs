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

fn is_ascii_byte(byte: u8) -> bool {
    match byte {
        0x20..=0x7e | b'\n' => true,
        _ => false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);
impl ColorCode {
    fn new(foreground: Color, background: Color) ->ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}
impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            if is_ascii_byte(byte) {
                self.write_byte(byte);
            } else {
                self.write_byte(0xfe);
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_pos;
                
                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_char: byte,
                    color_code,
                };
                self.column_pos += 1;
            }
        }
    }

    fn new_line(&mut self){}
}

pub fn test_print() {
    let mut writer = Writer {
        column_pos: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {&mut *(VGA_BUFFER_ADDR as *mut Buffer)},
    };

    writer.write_string("Jesus It worked\n");
}

const VGA_BUFFER_ADDR: u32 = 0xb8000;

static HELLO :&[u8] = b"Hello World!";

#[no_mangle]
pub fn unsafe_vga_print() -> ! {
    let vga_buffer = VGA_BUFFER_ADDR as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

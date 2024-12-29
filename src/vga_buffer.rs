use core::cmp::{max, min};
use core::fmt;
use core::num::NonZero;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::interrupts;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub column_position: usize,
    color_code: ColorCode,
    blink_color_code: ColorCode,
    blink_counter: u8,
    is_blink: bool,
    blink_frequency: NonZero<u8>,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(
        color_code: ColorCode,
        blink_color_code: ColorCode,
        blink_frequency: u8,
        buffer: &'static mut Buffer,
    ) -> Writer {
        Writer {
            column_position: 0,
            color_code,
            blink_color_code,
            blink_counter: 0,
            is_blink: true,
            blink_frequency: NonZero::new(blink_frequency).unwrap_or(NonZero::new(1).unwrap()),
            buffer,
        }
    }

    pub fn w_blink(&mut self) {
        self.blink_counter = self.blink_counter.wrapping_add(1);
        if self.blink_counter >= self.blink_frequency.get() - 1 {
            self.blink_counter = 0;
            self.is_blink = !self.is_blink;

            let column_position = min(self.column_position, BUFFER_WIDTH - 1);
            let mut current = self.buffer.chars[BUFFER_HEIGHT - 1][column_position].read();
            if self.is_blink {
                current.color_code = self.color_code;
            } else {
                current.color_code = self.blink_color_code;
            }
            self.buffer.chars[BUFFER_HEIGHT - 1][column_position].write(current);
        }
    }

    pub fn remove_blink(&mut self) {
        let column_position = min(self.column_position, BUFFER_WIDTH - 1);
        let mut current = self.buffer.chars[BUFFER_HEIGHT - 1][column_position].read();
        current.color_code = self.color_code;
        self.buffer.chars[BUFFER_HEIGHT - 1][column_position].write(current);
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        self.remove_blink();
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(
        ColorCode::new(Color::Yellow, Color::Black),
        ColorCode::new(Color::Yellow, Color::White),
        7,
        unsafe { &mut *(0xb8000 as *mut Buffer) }
    ));
}

pub fn blink() {
    WRITER.lock().w_blink();
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}

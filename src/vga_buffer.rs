// #![feature(const_fn)] // use unstable const function
// #![feature(unique)] 

#[allow(dead_code)] // suppress warning
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
}


//implement Copy trait, and its requisite
#[derive(Debug, Clone, Copy)] 
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        return ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode
}

const BUFFER_HEIGHT : usize = 25;
const BUFFER_WIDTH : usize = 80;

use volatile::Volatile;
struct Buffer {
    chars : [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

use core::ptr::Unique;

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8){
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer().chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code
                });
                self.column_position += 1;
            }
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        // converts raw pointer to safe mutable buffer reference
        // wraping?
        unsafe { self.buffer.get_mut() }
    }

    fn new_line(&mut self){ 
        //move all current character 1 line upper row
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let buffer = self.buffer();
                let character = buffer.chars[row][col].read();
                buffer.chars[row-1][col].write(character);
            }
        }
        //remove old row
        self.clear_row(BUFFER_HEIGHT-1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        //write 1 row out with blank character
        for col in 0..BUFFER_WIDTH {
            self.buffer().chars[row][col].write(blank);
        }
    }
}

use core::fmt;

//implement Write traint to support Rust's formatting macro
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result  {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}

//define static writer object, which must be thread safe
use spin::Mutex; //we cannot support threads or blocking feature yet. using spin
pub static WRITER: Mutex<Writer> = Mutex::new(Writer{
    column_position: 0,
    color_code: ColorCode::new(Color::LightGreen, Color::Black),
    buffer: unsafe { Unique::new(0xb8000 as *mut _ )}
});


//just copy
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

//wrap standard print macro to fit to our VGA adaptor
macro_rules! print {
    ($($arg:tt)*) => ({
        // to avoid deadlocking, let it eval $($arg)* first.
        $crate::vga_buffer::print(format_args!($($arg)*));
    });
}

pub fn print(args: fmt::Arguments){
    // the core of print! macro.
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        println!("");
    }
}

// pub fn print_something() {
    // use core::fmt::Write;
    // let mut writer = Writer {
        // column_position: 0,
        // color_code: ColorCode::new(Color::LightGreen, Color::Black),
        // buffer: unsafe { Unique::new(0xb8000 as *mut _) }
    // };

    // writer.write_byte(b'H');
    // writer.write_str("ello! ");
    // write!(writer, "The numbers are {} and {}", 42, 1.0/3.0);
// }

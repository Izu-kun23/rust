//! VGA Text Mode Buffer
//! 
//! Provides simple text output for kernel messages.
//! In production, this would be replaced with a proper graphics driver.

use core::fmt;

const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

// Simple static writer instance
// In bare-metal context, accessing static mut is common and necessary
static mut WRITER: Writer = Writer {
    column_position: 0,
    row_position: 0,
};

struct Writer {
    column_position: usize,
    row_position: usize,
}

impl Writer {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= VGA_WIDTH {
                    self.new_line();
                }

                let color_code = 0x1F; // Blue background, white foreground (like macOS)
                let offset = self.row_position * VGA_WIDTH + self.column_position;
                
                if offset < VGA_WIDTH * VGA_HEIGHT {
                    unsafe {
                        *VGA_BUFFER.add(offset) = (color_code as u16) << 8 | byte as u16;
                    }
                }
                
                self.column_position += 1;
            }
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    fn new_line(&mut self) {
        self.column_position = 0;
        if self.row_position < VGA_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            // Scroll up
            self.scroll();
        }
    }

    fn scroll(&mut self) {
        unsafe {
            // Move all lines up by one
            for row in 1..VGA_HEIGHT {
                for col in 0..VGA_WIDTH {
                    let src_idx = row * VGA_WIDTH + col;
                    let dst_idx = (row - 1) * VGA_WIDTH + col;
                    *VGA_BUFFER.add(dst_idx) = *VGA_BUFFER.add(src_idx);
                }
            }
            // Clear last line
            let last_row = VGA_HEIGHT - 1;
            let color_code = 0x1F;
            for col in 0..VGA_WIDTH {
                let idx = last_row * VGA_WIDTH + col;
                *VGA_BUFFER.add(idx) = (color_code as u16) << 8 | b' ' as u16;
            }
        }
    }

    pub fn clear_screen(&mut self) {
        let color_code = 0x1F; // Blue background
        unsafe {
            for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
                *VGA_BUFFER.add(i) = (color_code as u16) << 8 | b' ' as u16;
            }
        }
        self.column_position = 0;
        self.row_position = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// Suppress warnings for bare-metal static mut access
#[allow(static_mut_refs)]
pub fn clear_screen() {
    unsafe {
        WRITER.clear_screen();
    }
}

#[allow(static_mut_refs)]
pub fn print_fmt(args: fmt::Arguments) -> fmt::Result {
    use core::fmt::Write;
    unsafe {
        WRITER.write_fmt(args)
    }
}

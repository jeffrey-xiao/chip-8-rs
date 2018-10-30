pub const STANDARD_SCREEN_HEIGHT: usize = 32;
pub const STANDARD_SCREEN_WIDTH: usize = 64;
pub const SUPER_SCREEN_HEIGHT: usize = STANDARD_SCREEN_HEIGHT * 2;
pub const SUPER_SCREEN_WIDTH: usize = STANDARD_SCREEN_WIDTH * 2;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ScreenMode {
    Standard,
    Super,
}

pub struct Screen {
    pub mode: ScreenMode,
    pub pixels: [u8; SUPER_SCREEN_HEIGHT * SUPER_SCREEN_WIDTH / 8],
}

impl Screen {
    pub fn get_pixel(&self, row: usize, col: usize) -> bool {
        let index = row * self.width() + col;
        let byte_index = index / 8;
        let bit_index = index % 8;
        self.pixels[byte_index] & (1 << bit_index) != 0
    }

    pub fn flip_pixel(&mut self, row: usize, col: usize) {
        let index = row * self.width() + col;
        let byte_index = index / 8;
        let bit_index = index % 8;
        self.pixels[byte_index] ^= 1 << bit_index;
    }

    pub fn scroll_down(&mut self, rows: usize) {
        let row_bytes = self.width() / 8;
        for row in (0..self.height()).rev() {
            for col in 0..row_bytes {
                let index = row * row_bytes + col;
                if index >= row_bytes * rows {
                    self.pixels[index] = self.pixels[index - row_bytes * rows];
                } else {
                    self.pixels[index] = 0;
                }
            }
        }
    }

    pub fn scroll_right(&mut self) {
        let row_bytes = self.width() / 8;
        for row in 0..self.height() {
            for col in (1..row_bytes).rev() {
                self.pixels[row * row_bytes + col] <<= 4;
                self.pixels[row * row_bytes + col] |= self.pixels[row * row_bytes + col - 1] >> 4;
            }
            self.pixels[row * row_bytes] <<= 4;
        }
    }

    pub fn scroll_left(&mut self) {
        let row_bytes = self.width() / 8;
        for row in 0..self.height() {
            for col in 0..row_bytes - 1 {
                self.pixels[row * row_bytes + col] >>= 4;
                self.pixels[row * row_bytes + col] |= self.pixels[row * row_bytes + col + 1] << 4;
            }
            self.pixels[(row + 1) * row_bytes - 1] >>= 4;
        }
    }

    pub fn clear_screen(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = 0;
        }
    }

    pub fn pixels(&self) -> *const u8 {
        self.pixels.as_ptr()
    }

    pub fn width(&self) -> usize {
        match self.mode {
            ScreenMode::Standard => STANDARD_SCREEN_WIDTH,
            ScreenMode::Super => SUPER_SCREEN_WIDTH,
        }
    }

    pub fn height(&self) -> usize {
        match self.mode {
            ScreenMode::Standard => STANDARD_SCREEN_HEIGHT,
            ScreenMode::Super => SUPER_SCREEN_HEIGHT,
        }
    }

    pub fn get_mode(&mut self) -> ScreenMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: ScreenMode) {
        self.mode = mode;
    }
}

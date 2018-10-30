const KEY_COUNT: usize = 16;

pub struct Keypad {
    pub keys: u16,
}

impl Keypad {
    pub fn clear(&mut self) {
        self.keys = 0;
    }

    pub fn is_pressed(&self, index: usize) -> bool {
        (self.keys & (1 << index)) != 0
    }

    pub fn poll_key(&self) -> Option<usize> {
        for i in 0..KEY_COUNT {
            if self.is_pressed(i) {
                return Some(i);
            }
        }
        None
    }

    pub fn press_key(&mut self, index: usize) {
        self.keys |= 1 << index;
    }

    pub fn release_key(&mut self, index: usize) {
        self.keys &= !(1 << index);
    }
}

//! # chipo-rs
//!
//! [![NPM version](https://img.shields.io/npm/v/chipo.svg?style=flat)](https://www.npmjs.com/package/chipo)
//! [![chipo](http://meritbadge.herokuapp.com/chipo)](https://crates.io/crates/chipo)
//! [![Documentation](https://docs.rs/chipo/badge.svg)](https://docs.rs/chipo)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! [![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
//! [![Build Status](https://travis-ci.org/jeffrey-xiao/chipo-rs.svg?branch=master)](https://travis-ci.org/jeffrey-xiao/chipo-rs)
//! [![codecov](https://codecov.io/gh/jeffrey-xiao/chipo-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/jeffrey-xiao/chipo-rs)
//!
//! CHIP-8 Oxidized is a CHIP-8/SCHIP emulator written in Rust that can compile to WebAssembly.
//!
//! ## JavaScript Usage
//!
//! Install `chipo` from [npm](https://www.npmjs.com/):
//!
//! ```text
//! $ npm install chipo
//! ```
//!
//! Example JavaScript usage: [`chipo-web`](https://gitlab.com/jeffrey-xiao/chipo-web).
//!
//! ## Rust Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! chipo = "*"
//! ```
//!
//! and this to your crate root if you are using Rust 2015:
//!
//! ```rust
//! extern crate chipo;
//! ```
//!
//! ## Changelog
//!
//! See [CHANGELOG](CHANGELOG.md) for more details.
//!
//! ## References
//!
//! - [Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
//! - [Erik Bryntse's SUPER-CHIP v1.1](http://devernay.free.fr/hacks/chip8/schip.txt)
//!
//! ## License
//!
//! `chipo-rs` is distributed under the terms of both the MIT License and the Apache License
//! (Version 2.0).
//!
//! See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for more details.

#![warn(missing_docs)]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        fn generate_u8() -> u8 {
            (js_sys::Math::random() * 256.0).floor() as u8
        }
    } else {
        fn generate_u8() -> u8 {
            rand::thread_rng().gen()
        }
    }
}

mod keypad;
mod screen;

use crate::keypad::Keypad;
use crate::screen::{Screen, ScreenMode};
#[cfg(all(target_arch = "wasm32", console_error_panic_hook))]
use console_error_panic_hook::set_once;
#[cfg(not(target_arch = "wasm32"))]
use rand::Rng;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const REGISTER_COUNT: usize = 16;
const PROGRAM_START: u16 = 0x200;
const SUPER_MODE_RPL_FLAG_COUNT: usize = 8;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const SUPER_FONTSET: [u8; 160] = [
    0xFF, 0xFF, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, // 0
    0x18, 0x78, 0x78, 0x18, 0x18, 0x18, 0x18, 0x18, 0xFF, 0xFF, // 1
    0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, // 2
    0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 3
    0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, 0x03, 0x03, 0x03, 0x03, // 4
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 5
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, // 6
    0xFF, 0xFF, 0x03, 0x03, 0x06, 0x0C, 0x18, 0x18, 0x18, 0x18, // 7
    0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, // 8
    0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 9
    0x7E, 0xFF, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xC3, // A
    0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, // B
    0x3C, 0xFF, 0xC3, 0xC0, 0xC0, 0xC0, 0xC0, 0xC3, 0xFF, 0x3C, // C
    0xFC, 0xFE, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFE, 0xFC, // D
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, // E
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xC0, 0xC0, // F
];

#[derive(Copy, Clone, Eq, PartialEq)]
enum DrawMode {
    Clip,
    Wrap,
}

/// A chip-8 emulator.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Chip8 {
    screen: Screen,
    memory: [u8; MEMORY_SIZE],
    registers: [u8; REGISTER_COUNT],
    index: u16,
    pc: u16,
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; STACK_SIZE],
    sp: u16,
    keypad: Keypad,
    super_mode_rpl_flags: [u8; SUPER_MODE_RPL_FLAG_COUNT],
    should_draw: bool,
    should_beep: bool,
    is_running: bool,
    draw_mode: DrawMode,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Chip8 {
    /// Constructs a new `Chip8`.
    pub fn new() -> Self {
        #[cfg(all(target_arch = "wasm32", console_error_panic_hook))]
        set_once();

        Chip8 {
            screen: Screen::new(),
            memory: [0; MEMORY_SIZE],
            registers: [0; REGISTER_COUNT],
            index: 0,
            pc: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
            keypad: Keypad::new(),
            super_mode_rpl_flags: [0; SUPER_MODE_RPL_FLAG_COUNT],
            should_draw: false,
            should_beep: false,
            is_running: true,
            draw_mode: DrawMode::Wrap,
        }
    }

    fn initialize(&mut self) {
        for i in self.memory.iter_mut() {
            *i = 0;
        }

        let fontset_range = 0..FONTSET.len();
        for (d, s) in self.memory[fontset_range].iter_mut().zip(FONTSET.iter()) {
            *d = *s;
        }

        let super_fontset_range = FONTSET.len()..FONTSET.len() + SUPER_FONTSET.len();
        for (d, s) in self.memory[super_fontset_range]
            .iter_mut()
            .zip(SUPER_FONTSET.iter())
        {
            *d = *s;
        }

        for i in self.registers.iter_mut() {
            *i = 0;
        }

        self.index = 0;
        self.pc = PROGRAM_START;
        self.screen.clear_screen();
        self.screen.set_mode(ScreenMode::Standard);
        self.should_draw = true;
        self.delay_timer = 0;
        self.sound_timer = 0;

        for i in self.stack.iter_mut() {
            *i = 0;
        }

        self.sp = 0;

        self.keypad.clear();

        for i in self.super_mode_rpl_flags.iter_mut() {
            *i = 0;
        }

        self.is_running = true;
    }

    /// Loads a rom and sets the drawing mode of the emulator. If `should_wrap` is true, then all
    /// pixels drawn outside of the drawable area will wrap to the other side, else they will be
    /// ignored.
    pub fn load_rom(&mut self, rom: &[u8], should_wrap: bool) {
        self.initialize();
        self.draw_mode = {
            if should_wrap {
                DrawMode::Wrap
            } else {
                DrawMode::Clip
            }
        };

        for (i, byte) in rom.iter().enumerate() {
            self.memory[i + PROGRAM_START as usize] = *byte;
        }
    }

    fn fetch_opcode(&self) -> u16 {
        (u16::from(self.memory[self.pc as usize]) << 8)
            | u16::from(self.memory[(self.pc + 1) as usize])
    }

    /// Runs one fetch-decode-execute cycle.
    pub fn execute_cycle(&mut self) {
        if !self.is_running {
            return;
        }
        let opcode = self.fetch_opcode();
        self.pc += 2;

        self.process_opcode(opcode);
    }

    /// Decrement the delay and sound timer by one tick.
    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            if self.sound_timer == 0 {
                self.should_beep = true;
            }
        }
    }

    fn process_opcode(&mut self, opcode: u16) {
        self.should_beep = false;
        self.should_draw = false;

        let tokens = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            opcode & 0x000F,
        );

        let x = tokens.1 as usize;
        let y = tokens.2 as usize;
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as usize;

        match tokens {
            (0x0, 0x0, 0xC, _) => {
                self.screen.scroll_down(n);
                self.should_draw = true;
            }
            (0x0, 0x0, 0xE, 0x0) => {
                self.screen.clear_screen();
                self.should_draw = true;
            }
            (0x0, 0x0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            (0x0, 0x0, 0xF, 0xB) => {
                self.screen.scroll_right();
                self.should_draw = true;
            }
            (0x0, 0x0, 0xF, 0xC) => {
                self.screen.scroll_left();
                self.should_draw = true;
            }
            (0x0, 0x0, 0xF, 0xD) => self.is_running = false,
            (0x0, 0x0, 0xF, 0xE) => self.screen.set_mode(ScreenMode::Standard),
            (0x0, 0x0, 0xF, 0xF) => self.screen.set_mode(ScreenMode::Super),
            (0x1, _, _, _) => self.pc = nnn,
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            (0x3, _, _, _) => {
                if self.registers[x] == kk {
                    self.pc += 2;
                }
            }
            (0x4, _, _, _) => {
                if self.registers[x] != kk {
                    self.pc += 2;
                }
            }
            (0x5, _, _, 0x0) => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                }
            }
            (0x6, _, _, _) => self.registers[x] = kk,
            (0x7, _, _, _) => self.registers[x] = self.registers[x].wrapping_add(kk),
            (0x8, _, _, 0x0) => self.registers[x] = self.registers[y],
            (0x8, _, _, 0x1) => self.registers[x] |= self.registers[y],
            (0x8, _, _, 0x2) => self.registers[x] &= self.registers[y],
            (0x8, _, _, 0x3) => self.registers[x] ^= self.registers[y],
            (0x8, _, _, 0x4) => {
                let (res, overflow) = self.registers[x].overflowing_add(self.registers[y]);
                self.registers[x] = res;
                if overflow {
                    self.registers[15] = 1;
                } else {
                    self.registers[15] = 0;
                }
            }
            (0x8, _, _, 0x5) => {
                let (res, underflow) = self.registers[x].overflowing_sub(self.registers[y]);
                self.registers[x] = res;
                if underflow {
                    self.registers[15] = 0;
                } else {
                    self.registers[15] = 1;
                }
            }
            (0x8, _, _, 0x6) => {
                self.registers[15] = self.registers[x] & 1;
                self.registers[x] >>= 1;
            }
            (0x8, _, _, 0x7) => {
                let (res, underflow) = self.registers[y].overflowing_sub(self.registers[x]);
                self.registers[x] = res;
                if underflow {
                    self.registers[15] = 0;
                } else {
                    self.registers[15] = 1;
                }
            }
            (0x8, _, _, 0xE) => {
                self.registers[15] = self.registers[x] >> 7;
                self.registers[x] <<= 1;
            }
            (0x9, _, _, 0x0) => {
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => self.index = nnn,
            (0xB, _, _, _) => self.pc = u16::from(self.registers[0]) + nnn,
            (0xC, _, _, _) => {
                self.registers[x] = generate_u8() & kk;
            }
            (0xD, _, _, _) => {
                self.registers[15] = 0;

                let (rows, cols) = {
                    if self.screen.get_mode() == ScreenMode::Super && n == 0 {
                        (16, 16)
                    } else {
                        (n, 8)
                    }
                };

                for row in 0..rows {
                    for col in 0..cols {
                        let col_index = cols / 8;
                        let bitcode = self.memory[self.index as usize + row * col_index + col / 8];
                        if bitcode & (0x80 >> (col % 8)) == 0 {
                            continue;
                        }

                        let mut row = self.registers[y] as usize + row;
                        let mut col = self.registers[x] as usize + col;
                        match self.draw_mode {
                            DrawMode::Clip => {
                                if row > self.screen.height() || col > self.screen.width() {
                                    continue;
                                }
                            }
                            DrawMode::Wrap => {
                                row %= self.screen.height();
                                col %= self.screen.width();
                            }
                        }

                        if self.screen.get_pixel(row, col) {
                            self.registers[15] = 1;
                        }
                        self.screen.flip_pixel(row, col);
                    }
                }

                self.should_draw = true;
            }
            (0xE, _, 0x9, 0xE) => {
                if self.keypad.is_pressed(self.registers[x] as usize) {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA, 0x1) => {
                if !self.keypad.is_pressed(self.registers[x] as usize) {
                    self.pc += 2;
                }
            }
            (0xF, _, 0x0, 0x7) => self.registers[x] = self.delay_timer,
            (0xF, _, 0x0, 0xA) => {
                self.pc -= 2;
                if let Some(index) = self.keypad.poll_key() {
                    self.registers[x] = index as u8;
                    self.pc += 2;
                }
            }
            (0xF, _, 0x1, 0x5) => self.delay_timer = self.registers[x],
            (0xF, _, 0x1, 0x8) => self.sound_timer = self.registers[x],
            (0xF, _, 0x1, 0xE) => self.index += u16::from(self.registers[x]),
            (0xF, _, 0x2, 0x9) => self.index = u16::from(self.registers[x]) * 5,
            (0xF, _, 0x3, 0x0) => self.index = u16::from(self.registers[x]) * 10 + 80,
            (0xF, _, 0x3, 0x3) => {
                self.memory[self.index as usize] = self.registers[x] / 100;
                self.memory[self.index as usize + 1] = ((self.registers[x]) / 10) % 10;
                self.memory[self.index as usize + 2] = self.registers[x] % 10;
            }
            (0xF, _, 0x5, 0x5) => {
                for i in 0..=x {
                    self.memory[self.index as usize + i] = self.registers[i];
                }
            }
            (0xF, _, 0x6, 0x5) => {
                for i in 0..=x {
                    self.registers[i] = self.memory[self.index as usize + i];
                }
            }
            (0xF, _, 0x7, 0x5) => {
                self.super_mode_rpl_flags[..=x].clone_from_slice(&self.registers[..=x])
            }
            (0xF, _, 0x8, 0x5) => {
                self.registers[..=x].clone_from_slice(&self.super_mode_rpl_flags[..=x])
            }
            _ => panic!("Unrecognized opcode: {}", opcode),
        }
    }

    /// Returns a pointer to a byte array that represents the screen. The screen will have
    /// `screen_width * screen_height / 8` bytes in row-major order. Each byte represents 8 bits in
    /// little-endian. `1` represents that the pixel is black, while `0` represents that the pixel
    /// is white.
    pub fn screen(&self) -> *const u8 {
        self.screen.pixels()
    }

    /// Returns the width of the screen in pixels.
    pub fn screen_width(&self) -> usize {
        self.screen.width()
    }

    /// Returns the height of the screen in pixels.
    pub fn screen_height(&self) -> usize {
        self.screen.height()
    }

    /// Sets the state of a key to be pressed. `index` is the index of the key in row-major order.
    pub fn press_key(&mut self, index: usize) {
        self.keypad.press_key(index);
    }

    /// Sets the state of a key to be released. `index` is the index of the key in row-major order.
    pub fn release_key(&mut self, index: usize) {
        self.keypad.release_key(index);
    }

    /// Returns `true` if the screen has been updated and should be redrawn.
    pub fn should_draw(&self) -> bool {
        self.should_draw
    }

    /// Returns `true` if the a beep should be made.
    pub fn should_beep(&self) -> bool {
        self.should_beep
    }

    /// Returns the value of the program counter register.
    pub fn program_counter(&self) -> u16 {
        self.pc
    }

    /// Returns the value of the index register.
    pub fn index(&self) -> u16 {
        self.index
    }

    /// Returns a pointer to a byte array that represents the 16 data registers.
    pub fn registers(&self) -> *const u8 {
        self.registers.as_ptr()
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::fs;
    use std::hash::Hasher;
    use std::slice;

    #[test]
    fn test_rom() {
        let buffer = fs::read("tests/TEST_ROM").expect("Expected TEST_ROM to exist.");
        let mut chip_8 = Chip8::new();
        chip_8.load_rom(&buffer, true);

        for _ in 0..193 {
            chip_8.execute_cycle();
        }

        let screen = unsafe {
            slice::from_raw_parts(
                chip_8.screen(),
                chip_8.screen_height() * chip_8.screen_width(),
            )
        };

        let mut hasher = DefaultHasher::new();
        for val in screen {
            hasher.write_u8(*val);
        }

        assert_eq!(hasher.finish(), 0xA309_E966_20E3_20C5);
    }
}

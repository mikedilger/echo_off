// Copyright © 2015 by Optimal Computing Limited (of New Zealand)
// This code is licensed under the MIT license (see LICENSE-MIT for details)

extern crate libc;

use libc::{c_int,c_uint,c_uchar};

static STDIN_FILENO: c_int = 0;
static TCSAFLUSH: c_int = 2;
static ECHO: c_uint = 0o0000010;

/// Termios structure
#[repr(C)]
pub struct Termios {
    c_iflag: c_uint,
    c_oflag: c_uint,
    c_cflag: c_uint,
    c_lflag: c_uint,
    c_line: c_uchar,
    c_cc: [c_uchar; 32],
    c_ispeed: c_uint,
    c_ospeed: c_uint,
}
impl Termios {
    pub fn new() -> Termios {
        let mut t = Termios {
            c_iflag: 0, c_oflag: 0, c_cflag: 0, c_lflag: 0,
            c_line: 0,
            c_cc: [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
                   0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
            c_ispeed: 0, c_ospeed: 0,
        };
        unsafe {
            if tcgetattr(STDIN_FILENO, &mut t) == -1 {
                panic!("Could not call tcgetattr");
            }
        }
        t
    }

    /// Turn echo on
    pub fn echo_on(&mut self) -> () {
        self.c_lflag |= ECHO;
        unsafe {
            if tcsetattr(STDIN_FILENO, TCSAFLUSH, self) == -1 {
                panic!("Could not call tcsetattr");
            }
        }
    }

    /// Turn echo off
    pub fn echo_off(&mut self) -> () {
        self.c_lflag &= !ECHO;
        unsafe {
            if tcsetattr(STDIN_FILENO, TCSAFLUSH, self) == -1 {
                panic!("Could not call tcsetattr");
            }
        }
    }
}

#[link(name = "c")]
extern {
    fn tcgetattr(fd: c_int, termios: &mut Termios) -> c_int;
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios: &Termios)
        -> c_int;
}

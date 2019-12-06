#![no_std]
#![no_main]

use core::panic::PanicInfo;
use log::*;
use rand::{rngs::JitterRng, Rng};
use x86_64::instructions::port::Port;

struct Keyboard {
    port: Port<u8>,
}

impl Keyboard {
    fn new() -> Self {
        Self {
            port: Port::new(0x60),
        }
    }

    fn read(&self) -> Option<u8> {
        Some(unsafe { self.port.read() })
    }
}

fn tsc() -> u64 {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_rdtsc;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_rdtsc;

    unsafe { _rdtsc() as u64 }
}

struct Hardware {
    kbd: Keyboard,
    display: Display,
    rng: JitterRng,
    vramsz: (usize, usize),
}

impl Hardware {
    fn new(display: Display, kbd: Keyboard) -> Self {
        Self {
            display,
            kbd,
            rng: JitterRng::new_with_timer(tsc),
            vramsz: (0, 0),
        }
    }
}

impl libchip8::Hardware for Hardware {
    fn rand(&mut self) -> u8 {
        self.rng.gen()
    }

    fn key(&mut self, key: u8) -> bool {
        match self.kbd.read() {
            Some(r) => {
                debug!("{} down detected", r);

                r == match key {
                    0 => 0x2d,   // x
                    1 => 0x02,   // 1
                    2 => 0x03,   // 2
                    3 => 0x04,   // 3
                    4 => 0x10,   // q
                    5 => 0x11,   // w
                    6 => 0x12,   // e
                    7 => 0x1e,   // a
                    8 => 0x1f,   // s
                    9 => 0x20,   // d
                    0xa => 0x2c, // z
                    0xb => 0x2e, // c
                    0xc => 0x05, // 4
                    0xd => 0x12, // e
                    0xe => 0x20, // d
                    0xf => 0x2e, // c
                    _ => return false,
                }
            }
            None => return false,
        }
    }

    fn vram_set(&mut self, x: usize, y: usize, d: bool) {
        self.display.set(x, y, d);
    }

    fn vram_get(&mut self, x: usize, y: usize) -> bool {
        self.display.get(x, y)
    }

    fn vram_setsize(&mut self, size: (usize, usize)) {
        self.vramsz = size;
    }

    fn vram_size(&mut self) -> (usize, usize) {
        self.vramsz
    }

    fn clock(&mut self) -> u64 {
        tsc() / 2
    }

    fn beep(&mut self) {}

    fn sched(&mut self) -> bool {
        let t = self.clock();

        while self.clock().wrapping_sub(t) < 1000000 {}

        false
    }
}

struct Display {
    vram: *mut u8,
}

impl Display {
    const WIDTH: usize = 320;
    const SCALE: usize = 4;

    fn new() -> Self {
        Self {
            vram: 0xa0000 as *mut u8,
        }
    }

    fn set(&self, x: usize, y: usize, b: bool) {
        // let i = x + y * WIDTH;
        let s = Display::SCALE;
        let col = if b { 0x0f } else { 0x00 };
        for xo in 0..s {
            for yo in 0..s {
                let i = (x * s + xo) + (y * s + yo) * Display::WIDTH;

                unsafe {
                    *self.vram.offset(i as isize) = col;
                }
            }
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        let s = Display::SCALE;
        let i = x * s + y * s * Display::WIDTH;
        unsafe { *self.vram.offset(i as isize) == 0xf }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let d = Display::new();
    let k = Keyboard::new();

    let hw = Hardware::new(d, k);
    let chip8 = libchip8::Chip8::new(hw);
    chip8.run(include_bytes!("roms/invaders.ch8"));

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

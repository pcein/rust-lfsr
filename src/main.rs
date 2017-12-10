
// Do not do a release build if you want to see
// flashing LED's! The "delay" function in the "led"
// module will get optimized away during a release
// build.

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate volatile_register;

mod led;

use cortex_m::asm;
use core::iter;
use led::{portf_init, red_led, green_led, blue_led};

struct Lfsr {
    start: u16,
}

impl Iterator for Lfsr {
    type Item = u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        let bit  = ((self.start >> 0) ^ 
                    (self.start >> 2) ^ 
                    (self.start >> 3) ^ 
                    (self.start >> 5)) & 1;

        self.start =  (self.start >> 1) | (bit << 15);
        Some(bit)
    }
}

fn new_lfsr(n: u16) -> Lfsr {
    Lfsr { start: n }
}

fn main() {
    portf_init();
    let led = red_led();

    let l = new_lfsr(0x1234);

    for bit in l {
        if bit == 0 {
            led.off();
        } else {
            led.on();
        }
        led::delay(10000);
    }
    
}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}

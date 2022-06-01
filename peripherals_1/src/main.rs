#![no_std]
#![no_main]

// pick a panicking behavior
use crate::tm4c129x::gpio_porta_ahb::_PC;
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

//use cortex_m::asm;
use cortex_m_rt::entry;
use tm4c129x::{
    self,
    generic::{Readable, Reg},
    gpio_porta_ahb::{_DEN, _DIR, _PUR},
    sysctl::_RCGCGPIO,
};

fn read_bits<T>(reg: &Reg<u32, T>) -> u32
where
    Reg<u32, T>: Readable,
{
    let bits = reg.read().bits();
    return bits;
}

fn set_bits_rcgcgpio(reg: &Reg<u32, _RCGCGPIO>, bit: &[u8]) {
    let mut bits = read_bits(reg);
    for b in bit {
        bits |= 1 << b;
    }
    reg.write(|w| unsafe { w.bits(bits) });
}

fn set_bits_dir(reg: &Reg<u32, _DIR>, bit: &[u8]) {
    let mut bits = read_bits(reg);
    for b in bit {
        if b < &0x80 {
            bits |= 1 << b;
        } else {
            bits &= !(1 << !b);
        }
    }
    reg.write(|w| unsafe { w.bits(bits) });
}

fn set_bits_den(reg: &Reg<u32, _DEN>, bit: &[u8]) {
    let mut bits = read_bits(reg);
    for b in bit {
        bits |= 1 << b;
    }
    reg.write(|w| unsafe { w.bits(bits) });
}

fn set_bits_pur(reg: &Reg<u32, _PUR>, bit: &[u8]) {
    let mut bits = read_bits(reg);
    for b in bit {
        bits |= 1 << b;
    }
    reg.write(|w| unsafe { w.bits(bits) });
}

fn set_bits_pc(reg: &Reg<u32, _PC>, bit: &[u8]) {
    let mut bits = read_bits(reg);
    for b in bit {
        bits |= 1 << b;
    }
    reg.write(|w| unsafe { w.bits(bits) });
}

#[entry]
fn main() -> ! {
    let p = tm4c129x::Peripherals::take().unwrap();

    // Enable clock for ports F, J,N
    let bits_to_set = [5, 8, 12];
    set_bits_rcgcgpio(&p.SYSCTL.rcgcgpio, &bits_to_set);

    // D1 and D2 are connected to GPIOs PN1 and PN0. These
    // LEDs are dedicated for use by the software application
    let bits_to_set: [u8; 2] = [0, 1];
    set_bits_dir(&p.GPIO_PORTN.dir, &bits_to_set);
    set_bits_den(&p.GPIO_PORTN.den, &bits_to_set);

    // D3 and D4 are connected to GPIOs PF4 and PF0, which can be controlled by user’s
    // software or the integrated Ethernet module of the microcontroller.
    let bits_to_set: [u8; 2] = [0, 4];
    set_bits_dir(&p.GPIO_PORTF_AHB.dir, &bits_to_set);
    set_bits_den(&p.GPIO_PORTF_AHB.den, &bits_to_set);

    // Two user switches are provided for input and control of the TM4C1294NCPDTI software. The switches
    // are connected to GPIO pins PJ0 and PJ1.
    let bits_to_set: [u8; 2] = [!0, !1];
    set_bits_dir(&p.GPIO_PORTJ_AHB.dir, &bits_to_set);

    let bits_to_set: [u8; 2] = [0, 1];
    set_bits_den(&p.GPIO_PORTJ_AHB.den, &bits_to_set);
    // pull-up resistor
    set_bits_pur(&p.GPIO_PORTJ_AHB.pur, &bits_to_set);
    // ???
    set_bits_pc(&p.GPIO_PORTJ_AHB.pc, &bits_to_set);

    let mut lastsw1 = false;
    let mut lastsw2 = false;
    let mut outputmask = 0;
    loop {
        let bits = p.GPIO_PORTJ_AHB.data.read().bits();
        let sw1 = (bits & 0x01) != 0;
        let sw2 = (bits & 0x02) != 0;

        if sw1 {
            if !lastsw1 {
                outputmask += 1;
            }
        }
        lastsw1 = sw1;

        if sw2 {
            if !lastsw2 {
                outputmask |= 0x04;
                outputmask -= 1;
            }
        }
        lastsw2 = sw2;

        outputmask &= 0x03;
        p.GPIO_PORTN.data.write(|w| unsafe { w.bits(outputmask) });
    }
}

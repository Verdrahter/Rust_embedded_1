#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

//use cortex_m::asm;
use cortex_m_rt::entry;
use tm4c129x::{self, generic::Reg, sysctl::_RCGCGPIO};

fn set_bits(reg: &Reg<u32, _RCGCGPIO>, bit: &[u8]) {
    let mut bits = reg.read().bits();
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
    set_bits(&p.SYSCTL.rcgcgpio, &bits_to_set);

    // D1 and D2 are connected to GPIOs PN1 and PN0. These
    // LEDs are dedicated for use by the software application
    let mut bits = p.GPIO_PORTN.dir.read().bits();
    bits |= 0x03;
    p.GPIO_PORTN.dir.write(|w| unsafe { w.bits(bits) });

    bits = p.GPIO_PORTN.den.read().bits();
    bits |= 0x03;
    p.GPIO_PORTN.den.write(|w| unsafe { w.bits(bits) });

    // D3 and D4 are connected to GPIOs PF4 and PF0, which can be controlled by user’s
    // software or the integrated Ethernet module of the microcontroller.
    bits = p.GPIO_PORTF_AHB.dir.read().bits();
    bits |= 0x11;
    p.GPIO_PORTF_AHB.dir.write(|w| unsafe { w.bits(bits) });

    bits = p.GPIO_PORTF_AHB.den.read().bits();
    bits |= 0x11;
    p.GPIO_PORTF_AHB.den.write(|w| unsafe { w.bits(bits) });

    // Two user switches are provided for input and control of the TM4C1294NCPDTI software. The switches
    // are connected to GPIO pins PJ0 and PJ1.
    bits = p.GPIO_PORTJ_AHB.dir.read().bits();
    bits &= !0x03;
    p.GPIO_PORTJ_AHB.dir.write(|w| unsafe { w.bits(bits) });

    bits = p.GPIO_PORTJ_AHB.den.read().bits();
    bits |= 0x03;
    p.GPIO_PORTJ_AHB.den.write(|w| unsafe { w.bits(bits) });

    // pull-up resistor
    bits = p.GPIO_PORTJ_AHB.pur.read().bits();
    bits |= 0x03;
    p.GPIO_PORTJ_AHB.pur.write(|w| unsafe { w.bits(bits) });

    // ???
    p.GPIO_PORTJ_AHB.pc.write(|w| unsafe { w.bits(3) });

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

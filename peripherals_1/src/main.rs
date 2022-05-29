#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

//use cortex_m::asm;
use cortex_m_rt::entry;
use tm4c129x;

#[entry]
fn main() -> ! {
    let p = tm4c129x::Peripherals::take().unwrap();
    let sysctl = p.SYSCTL;
    let rcgcgpio = &sysctl.rcgcgpio;
    let mut bits = rcgcgpio.read().bits();
    bits |= 1<<5;  // Enable port F clock
    bits |= 1<<8;  // Enable port J clock
    bits |= 1<<12; // Enable port N clock
    rcgcgpio.write(|w| unsafe{ w.bits(bits) } );

    // D1 and D2 are connected to GPIOs PN1 and PN0. These
    // LEDs are dedicated for use by the software application
    bits = p.GPIO_PORTN.dir.read().bits();
    bits |= 0x03;
    p.GPIO_PORTN.dir.write(|w| unsafe{ w.bits(bits) });

    bits = p.GPIO_PORTN.den.read().bits();
    bits |= 0x03;
    p.GPIO_PORTN.den.write(|w| unsafe{ w.bits(bits) });

    // D3 and D4 are connected to GPIOs PF4 and PF0, which can be controlled by user’s
    // software or the integrated Ethernet module of the microcontroller.
    bits = p.GPIO_PORTF_AHB.dir.read().bits();
    bits |= 0x11;
    p.GPIO_PORTF_AHB.dir.write(|w| unsafe{ w.bits(bits) });

    bits = p.GPIO_PORTF_AHB.den.read().bits();
    bits |= 0x11;
    p.GPIO_PORTF_AHB.den.write(|w| unsafe{ w.bits(bits) });

    // Two user switches are provided for input and control of the TM4C1294NCPDTI software. The switches
    // are connected to GPIO pins PJ0 and PJ1.
    bits = p.GPIO_PORTJ_AHB.dir.read().bits();
    bits &= !0x03;
    p.GPIO_PORTJ_AHB.dir.write(|w| unsafe{ w.bits(bits) });

    bits = p.GPIO_PORTJ_AHB.den.read().bits();
    bits |= 0x03;
    p.GPIO_PORTJ_AHB.den.write(|w| unsafe{ w.bits(bits) });

    loop {
        let bits = p.GPIO_PORTJ_AHB.data.read().bits(); 
        let sw1 = (bits & 0x01) != 0;
        let sw2 = (bits & 0x02) != 0;

        if sw1 && sw2 {
            p.GPIO_PORTN.data.write(|w| unsafe{w.bits(0x02)});
        }
        else if sw1 {
            p.GPIO_PORTN.data.write(|w| unsafe{w.bits(0x01)});
        }
        else if sw2 {
            p.GPIO_PORTF_AHB.data.write(|w| unsafe{w.bits(0x10)});
        }
        else {
            p.GPIO_PORTF_AHB.data.write(|w| unsafe{w.bits(0x01)});

        }
    }
}

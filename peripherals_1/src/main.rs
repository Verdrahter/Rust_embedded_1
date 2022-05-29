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
    bits |= 1<<5 | 1<<8 | 1<<12; // Enable port F, J and N clocks
    rcgcgpio.write(|w| unsafe{ w.bits(bits) } );

    bits = p.GPIO_PORTN.dir.read().bits();
    bits |= 0x03;
    p.GPIO_PORTN.dir.write(|w| unsafe{ w.bits(bits) });

    bits = p.GPIO_PORTN.den.read().bits();
    bits |= 0x03;
    p.GPIO_PORTN.den.write(|w| unsafe{ w.bits(bits) });

    bits = p.GPIO_PORTF_AHB.dir.read().bits();
    bits |= 0x11;
    p.GPIO_PORTF_AHB.dir.write(|w| unsafe{ w.bits(bits) });

    bits = p.GPIO_PORTF_AHB.den.read().bits();
    bits |= 0x11;
    p.GPIO_PORTF_AHB.den.write(|w| unsafe{ w.bits(bits) });

    bits = p.GPIO_PORTJ_AHB.dir.read().bits();
    bits &= !0x03;
    p.GPIO_PORTJ_AHB.dir.write(|w| unsafe{ w.bits(bits) });

    bits = p.GPIO_PORTJ_AHB.den.read().bits();
    bits &= !0x03;
    p.GPIO_PORTJ_AHB.den.write(|w| unsafe{ w.bits(bits) });

    loop {
        let sw1 = (p.GPIO_PORTJ_AHB.dir.read().bits() & 0x01) != 0;
        let sw2 = (p.GPIO_PORTJ_AHB.dir.read().bits() & 0x02) != 0;

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

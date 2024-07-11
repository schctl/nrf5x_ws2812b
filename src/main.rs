#![no_std]
#![no_main]

pub mod device;
pub mod driver;

// ------- Panic Handler -------

use core::panic::PanicInfo;

use driver::Rgb;

#[inline(never)]
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

/// Start high speed crystal oscillator (64Mhz).
///
/// Our system's high speed clock will use the crystal as a clock source.
fn setup_hf_clock(clock: &device::CLOCK) {
    clock.tasks_hfclkstart.write(|w| unsafe { w.bits(1) });
    while clock.events_hfclkstarted.read().bits() == 0 {}
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let cpu_pphl = cortex_m::Peripherals::take().unwrap();
    let mcu_pphl = device::Peripherals::take().unwrap();

    setup_hf_clock(&mcu_pphl.CLOCK);

    let mut delay_provider = cortex_m::delay::Delay::new(cpu_pphl.SYST, 64_000_000);

    let ws2812b = driver::gpio::Driver::new(0, 23, &mcu_pphl);

    let leds = [Rgb::H_RED, Rgb::ZERO, Rgb::H_BLUE];

    loop {
        ws2812b.write(leds.into_iter(), &mut delay_provider);
        delay_provider.delay_ms(500);
    }
}

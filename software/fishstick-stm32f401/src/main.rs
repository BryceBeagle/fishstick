#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use defmt::{info, println};
#[allow(unused_imports)]
use defmt_rtt as _; // global_logger
#[allow(unused_imports)]
use panic_probe as _; // panic_handler
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    println!("Hello, world!");

    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioc = dp.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&clocks);

        loop {
            // On for 1s, off for 1s.
            info!("off");
            led.set_high();
            delay.delay_ms(1000_u32);

            info!("on");
            led.set_low();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOB peripheral
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // Configure pins as push-pull outputs. The `crh` register is passed to the function in order to
    // configure the port. For pins 0-7, crl should be passed instead.
    let mut red_led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let mut yel_led = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    let mut grn_led = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);

    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        block!(timer.wait()).unwrap();
        grn_led.set_high().unwrap();
        yel_led.set_low().unwrap();
        red_led.set_low().unwrap();
        block!(timer.wait()).unwrap();
        grn_led.set_low().unwrap();
        yel_led.set_high().unwrap();
        red_led.set_low().unwrap();
        block!(timer.wait()).unwrap();
        grn_led.set_low().unwrap();
        yel_led.set_low().unwrap();
        red_led.set_high().unwrap();
    }
}
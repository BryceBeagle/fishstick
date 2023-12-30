#![deny(unsafe_code)]
#![no_main]
#![no_std]

use defmt::info;
#[allow(unused_imports)] // global_logger
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};
#[allow(unused_imports)] // panic_handle
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    loop {
        info!("high");
        led.set_high();
        Timer::after(Duration::from_millis(2000)).await;

        info!("low");
        led.set_low();
        Timer::after(Duration::from_millis(2000)).await;
    }
}

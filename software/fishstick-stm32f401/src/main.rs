#![no_main]
#![no_std]

mod gamecube;

use embedded_hal::spi::MODE_0;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use stm32f4xx_hal::{gpio::NoPin, pac, prelude::*, spi::Spi, timer::Timer};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // get peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // set up clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .sysclk(64.MHz())
        .hclk(64.MHz())
        .pclk1(2.MHz())
        .freeze();

    // get pins
    let gpiob = dp.GPIOB.split();
    let pin_write = gpiob.pb15.into_alternate();

    // set up SPI
    let mut spi = Spi::new_bidi(dp.SPI2, (NoPin, NoPin, pin_write), MODE_0, 1.MHz(), &clocks);

    // set up timer for creating delays
    let mut timer = Timer::syst(cp.SYST, &clocks).delay();

    loop {
        // write status message to SPI for testing with gamecube controller
        rprintln!("Sending STATUS_MESSAGE");
        spi.write(gamecube::STATUS_MESSAGE).unwrap();
        timer.delay_ms(100u32);
    }
}

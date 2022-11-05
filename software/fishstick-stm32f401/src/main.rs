#![no_main]
#![no_std]

use embedded_hal::spi::MODE_0;
use nb::block;
use panic_rtt_target as _;
use rtt_target::rtt_init_default;

use cortex_m_rt::entry;
use stm32f4xx_hal::{gpio::NoPin, pac, prelude::*, spi::Spi, timer::Timer};

#[entry]
fn main() -> ! {
    rtt_init_default!();

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .sysclk(64.MHz())
        .hclk(32.MHz())
        .pclk1(2.MHz())
        .freeze();

    let gpiob = dp.GPIOB.split();
    let mosi = gpiob.pb15.into_alternate();
    let mut spi = Spi::new_bidi(dp.SPI2, (NoPin, NoPin, mosi), MODE_0, 1.MHz(), &clocks);

    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(60.Hz()).unwrap();

    let data = &[0b1010_1010, 0b1010_1010];

    loop {
        spi.write(data).unwrap();
        block!(timer.wait()).unwrap();
    }
}

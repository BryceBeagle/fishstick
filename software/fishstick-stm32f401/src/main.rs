#![no_main]
#![no_std]

use cortex_m_rt::entry;
#[allow(unused_imports)]
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::dma::traits::Stream;
use stm32f4xx_hal::timer::CCR;
use stm32f4xx_hal::{
    dma::{config::DmaConfig, StreamsTuple, Transfer},
    pac::{interrupt, Interrupt, TIM1},
    prelude::*,
    rcc::RccExt,
    timer::{Channel1, Channel2},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = stm32f4xx_hal::pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpioa = dp.GPIOA.split();
    let channels = (
        Channel1::new(gpioa.pa8.into_alternate()),
        Channel2::new(gpioa.pa9.into_alternate()),
    );

    let (mut pwm1, _pwm2) = dp.TIM1.pwm_hz(channels, 1.MHz() / 4, &clocks).split();
    pwm1.set_polarity(stm32f4xx_hal::timer::Polarity::ActiveHigh);

    let max_duty = pwm1.get_max_duty();
    let duty_25 = (max_duty / 4) as u16;
    let duty_75 = (duty_25 * 3) as u16;
    pwm1.set_duty(0);

    let mut stream = StreamsTuple::new(dp.DMA2).6;
    unsafe { stream.enable() }

    let buffer =
        cortex_m::singleton!(: [u16; 5] = [duty_25, duty_75, duty_25, duty_75, 0]).unwrap();

    let ccr1 = CoolCCR(dp.TIM1);
    let ccr1: CCR<TIM1, 0> = unsafe { core::intrinsics::transmute(ccr1) };

    let transfer = Transfer::init_memory_to_peripheral(
        stream,
        ccr1,
        buffer,
        None,
        DmaConfig::default()
            .memory_increment(true)
            .fifo_enable(true)
            .fifo_error_interrupt(true)
            .transfer_complete_interrupt(true),
    );

    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::DMA2_STREAM6);
    }

    pwm1.enable();

    loop {
        cortex_m::asm::nop();
    }
}

#[interrupt]
fn DMA2_STREAM6() {
    cortex_m::interrupt::free(|cs| {
        // let mut tref = GTRANSFER.borrow(cs).borrow_mut();
        // let Some(transfer) = tref.as_mut() else {
        //     rprintln!("I no get it");
        //     return;
        // };
        //
        // if Stream6::<DMA2>::get_transfer_complete_flag() {
        //     transfer.clear_transfer_complete_interrupt();
        //     rprintln!("Cleared Transfer Complete Flag");
        // }
        //
        // if Stream6::<DMA2>::get_fifo_error_flag() {
        //     transfer.clear_fifo_error_interrupt();
        //     rprintln!("Cleared FIFO Error Flag");
        // }
    });

    rprintln!("DONE");
}

struct CoolCCR<T>(T);

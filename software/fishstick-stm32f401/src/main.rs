// #![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    dma::{
        config::DmaConfig,
        traits::{DMASet, PeriAddress, StreamISR},
        MemoryToPeripheral, Stream6, StreamX, StreamsTuple, Transfer,
    },
    pac::{interrupt, Interrupt, DMA2, TIM1},
    prelude::*,
    rcc::RccExt,
    timer::{Channel1, Channel2},
};

type JustFkingWorkTransfer<'a> =
    Transfer<StreamX<DMA2, 6>, 0, JustFkingWork, MemoryToPeripheral, &'a mut [u32; 5]>;
static GTRANSFER: Mutex<RefCell<Option<JustFkingWorkTransfer<'static>>>> =
    Mutex::new(RefCell::new(None));

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
    let duty_25 = (max_duty / 4) as u32;
    let duty_75 = (duty_25 * 3) as u32;
    pwm1.set_duty(0);

    unsafe {
        let tim = &*TIM1::PTR;
        tim.dier.modify(|_, w| w.cc1de().set_bit());
    }

    let stream = StreamsTuple::new(dp.DMA2).6;
    let buffer =
        cortex_m::singleton!(: [u32; 5] = [duty_25, duty_75, duty_25, duty_75, 0]).unwrap();

    let mut transfer = Transfer::init_memory_to_peripheral(
        stream,
        JustFkingWork,
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

    cortex_m::interrupt::free(|cs| {
        transfer.start(|_| {
            rprintln!("DMA Started");
        });
        *GTRANSFER.borrow(cs).borrow_mut() = Some(transfer);
    });

    // unsafe {
    //     let tim1 = &*TIM1::PTR;
    //     let oc = tim1.ccmr1_output().read().oc1pe().is_disabled();
    //     panic!("{oc}");
    // }

    // let mut delay = 0;
    loop {
        // transfer.start(|_| {
        //     // rprintln!("DMA GOING AGAIN");
        // });
        unsafe {
            let tim1 = &*TIM1::PTR;
            // tim1.ccmr1_output().write(|w| w.oc1pe().set_bit());
            // let oc = tim1.ccmr1_output().read().oc1pe().is_disabled();
            // panic!("{oc}");

            let reg = tim1.ccr1().read().bits();
            // if delay > 100 {
            //     let ptr = tim1.ccr1().as_ptr();
            //     *ptr = *ptr + 1;
            //     delay = 0;
            // } else {
            //     delay += 1;
            // }

            if reg > 0 {
                rprintln!("{}", reg);
            }
        }
        cortex_m::asm::nop();
    }
}

#[interrupt]
fn DMA2_STREAM6() {
    cortex_m::interrupt::free(|cs| {
        let mut tref = GTRANSFER.borrow(cs).borrow_mut();
        let Some(transfer) = tref.as_mut() else {
            rprintln!("I no get it");
            return;
        };

        if Stream6::<DMA2>::get_transfer_complete_flag() {
            transfer.clear_transfer_complete_interrupt();
            rprintln!("Cleared Transfer Complete Flag");
        }

        if Stream6::<DMA2>::get_fifo_error_flag() {
            transfer.clear_fifo_error_interrupt();
            rprintln!("Cleared FIFO Error Flag");
        }
    });

    rprintln!("DONE");
}

struct JustFkingWork;

unsafe impl DMASet<StreamX<DMA2, 6>, 0, stm32f4xx_hal::dma::MemoryToPeripheral> for JustFkingWork {}

unsafe impl PeriAddress for JustFkingWork {
    type MemSize = u32;

    fn address(&self) -> u32 {
        unsafe {
            let tim1 = &*TIM1::PTR;
            tim1.ccr1().as_ptr() as u32
        }
    }
}

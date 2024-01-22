//! CDC-ACM serial port example using polling in a busy loop.
//! Target board: any STM32F4 with a OTG FS peripheral and a 25MHz HSE crystal
#![no_std]
#![no_main]

use defmt::info;
#[allow(unused_imports)] // global_logger
use defmt_rtt as _;
#[allow(unused_imports)] // panic_handle
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use stm32f4xx_hal::{interrupt, pac, prelude::*};
use usb_device::class_prelude::*;
use usb_device::prelude::*;
use usbd_hid::descriptor::{generator_prelude::*, MouseReport};
use usbd_hid::hid_class::HIDClass;

use cortex_m::asm::delay as cycle_delay;
use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

static mut USB_ALLOC: Option<UsbBusAllocator<UsbBus<USB>>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBus<USB>>> = None;
static mut USB_HID: Option<HIDClass<UsbBus<USB>>> = None;

fn push_mouse_movement(report: MouseReport) -> Result<usize, usb_device::UsbError> {
    disable_interrupts(|_| unsafe { USB_HID.as_mut().map(|hid| hid.push_input(&report)) }).unwrap()
}

fn poll_usb() {
    unsafe {
        if let (Some(usb_device), Some(usb_hid)) = (USB_DEVICE.as_mut(), USB_HID.as_mut()) {
            usb_device.poll(&mut [usb_hid]);
        }
    };
}

#[interrupt]
fn OTG_FS() {
    poll_usb();
}

#[entry]
fn main() -> ! {
    info!("heyyyyyyyyyyyyyy");

    let peripherals = pac::Peripherals::take().unwrap();
    let mut core = pac::CorePeripherals::take().unwrap();
    let rcc = peripherals.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(25.MHz())
        .sysclk(48.MHz())
        .require_pll48clk()
        .freeze();

    let gpioa = peripherals.GPIOA.split();

    let usb = USB::new(
        (
            peripherals.OTG_FS_GLOBAL,
            peripherals.OTG_FS_DEVICE,
            peripherals.OTG_FS_PWRCLK,
        ),
        (gpioa.pa11, gpioa.pa12),
        &clocks,
    );

    unsafe {
        USB_ALLOC = Some(UsbBus::new(usb, &mut EP_MEMORY));

        USB_HID = Some(HIDClass::new(
            USB_ALLOC.as_ref().unwrap(),
            MouseReport::desc(),
            60,
        ));

        USB_DEVICE = Some(
            UsbDeviceBuilder::new(USB_ALLOC.as_ref().unwrap(), UsbVidPid(0x16c0, 0x27dd))
                .strings(&[StringDescriptors::default()
                    .manufacturer("Fake Company")
                    .product("Product")
                    .serial_number("TEST")])
                .unwrap()
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::OTG_FS, 1);
        NVIC::unmask(interrupt::OTG_FS);
    }

    loop {
        cycle_delay(25 * 1024 * 1024);
        push_mouse_movement(MouseReport {
            x: 0,
            y: 4,
            buttons: 0,
            pan: 0,
            wheel: 0,
        })
        .ok()
        .unwrap_or(0);
        cycle_delay(25 * 1024 * 1024);
        push_mouse_movement(MouseReport {
            x: 0,
            y: -4,
            buttons: 0,
            pan: 0,
            wheel: 0,
        })
        .ok()
        .unwrap_or(0);
    }
}

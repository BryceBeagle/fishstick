#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

mod gamecube;
mod joybus;

#[entry]
fn main() -> ! {
    let jb = stm32::Joybus();
    let controller = gamecube::Controller::new(jb);

    controller.calibrate();

    loop {
        let status = controller.status();
    }
}

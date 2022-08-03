#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy::executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::Peripherals;
use embassy_stm32::peripherals::PC13;
use {defmt_rtt as _, panic_probe as _};

#[embassy::main]
async fn main(spawner: Spawner, p: Peripherals) {
    info!("Hello World!");

    let button = Input::new(p.PC13, Pull::Down);
    let button = ExtiInput::new(button, p.EXTI13);

    info!("Press the USER button...");

    unwrap!(spawner.spawn(btn(button)));

}

#[embassy::task]
async fn btn(mut button: ExtiInput<'static, PC13>) {
    loop {
        button.wait_for_rising_edge().await;
        info!("Pressed!");
        button.wait_for_falling_edge().await;
        info!("Released!");
    }
}

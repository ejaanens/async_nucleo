#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    peripherals::PB7,
    Peripherals,
};
use {defmt_rtt as _, panic_probe as _};

#[embassy::main]
async fn main(spawner: Spawner, p: Peripherals) {
    info!("Hello World!");

    let led = Output::new(p.PB7, Level::High, Speed::Low);
    unwrap!(spawner.spawn(blinker(led, Duration::from_micros(500000-274))));
}

#[embassy::task]
async fn blinker(mut led: Output<'static, PB7>, interval: Duration) {
    loop {
        info!("high");
        led.set_high();
        Timer::after(interval).await;

        info!("low");
        led.set_low();
        Timer::after(interval).await;
    }
}

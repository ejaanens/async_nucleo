#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::executor::Spawner;
use embassy_executor::time::{Duration, Timer};

use embassy_stm32::peripherals::{I2C1, DMA1_CH0, DMA1_CH1};
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    i2c::I2c,
    time::Hertz,
    // interrupt,
    peripherals::PB7,
    Peripherals,
};
use {defmt_rtt as _, panic_probe as _};

use async_nucleo::rtc_lib::*;

#[embassy_executor::main]
async fn main(spawner: Spawner, p: Peripherals) {
    info!("Hello World!");

    let irq = p.I2C1.into();

    let i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        irq,
        p.DMA1_CH0,
        p.DMA1_CH1,
        Hertz::khz(400),
    );

    let led = Output::new(p.PB7, Level::High, Speed::Low);

    let rtc = DS3231::new(i2c);

    unwrap!(spawner.spawn(blinker(led, Duration::from_micros(500000-213))));
    unwrap!(spawner.spawn(clock(rtc, Duration::from_millis(1000))));
}

#[embassy_executor::task]
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

#[embassy_executor::task] // with more i2c use mutex
async fn clock(mut rtc: DS3231<I2c<'static, I2C1, DMA1_CH0, DMA1_CH1>>, interval: Duration) {
    loop {
        if let Ok(time) = rtc.read_clock().await {

            info!("clock {:?}", time);
        }
        Timer::after(interval).await;
    }
}

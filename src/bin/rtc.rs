#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::{
    executor::Spawner,
    time::{Duration, Timer},
};
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;

use embassy_util::{
    Forever,
    blocking_mutex::raw::ThreadModeRawMutex,
    mutex::Mutex,
};

use embassy_stm32::{
    peripherals::{DMA1_CH2, DMA1_CH3, I2C2, PB7},
    gpio::{Level, Output, Speed},
    i2c::{self, I2c},
    interrupt::{self, I2C2_EV},
    time::Hertz,
    Peripherals,
};
use chrono::NaiveDate;
use {defmt_rtt as _, panic_probe as _};

use async_nucleo::rtc_lib::*;

static I2C2_BUS: Forever<Mutex::<ThreadModeRawMutex, I2c<'static, I2C2, DMA1_CH2, DMA1_CH3>>> = Forever::new();

#[embassy_executor::main]
async fn main(spawner: Spawner, p: Peripherals) {
    info!("Hello World!");


    let irq = interrupt::take!(I2C2_EV);

    let i2c2 = I2c::new(
        p.I2C2,
        p.PF1,
        p.PF0,
        irq,
        p.DMA1_CH2,
        p.DMA1_CH3,
        Hertz::khz(400),
        i2c::Config::default(),
    );

    let rtc = DS3231::new(i2c2);

    // let i2c2_bus = Mutex::<ThreadModeRawMutex, _>::new(i2c2);
    // let i2c2_bus = I2C2_BUS.put(i2c2_bus);

    unwrap!(spawner.spawn(blinker(p.PB7, Duration::from_micros(500000-213))));
    // unwrap!(spawner.spawn(clock(i2c2_bus, Duration::from_millis(1000))));

    let datetime = NaiveDate::from_ymd(2022, 8, 17).and_hms(20, 20, 20);
    rtc.set_datetime(&datetime).await.unwrap();

    loop {
        if let Ok(time) = rtc.time().await {

            info!("clock {:?}", time);
        }

        Timer::after(interval).await;
    }
}

#[embassy_executor::task]
async fn blinker(led: PB7, interval: Duration) {
    let mut led = Output::new(led, Level::High, Speed::Low);
    loop {
        info!("high");
        led.set_high();
        Timer::after(interval).await;

        info!("low");
        led.set_low();
        Timer::after(interval).await;
    }
}

#[embassy_executor::task]
async fn clock(mut i2c2_bus: &mut Mutex<ThreadModeRawMutex, I2c<'static, I2C2, DMA1_CH2, DMA1_CH3>>, interval: Duration) {
    let i2c2_dev0 = I2cDevice::new(i2c2_bus);

    let rtc = DS3231::new(i2c2_dev0);

    let datetime = NaiveDate::from_ymd(2022, 8, 17).and_hms(20, 20, 20);
    rtc.set_datetime(&datetime).await.unwrap();

    loop {
        if let Ok(time) = rtc.time().await {

            info!("clock {:?}", time);
        }

        Timer::after(interval).await;
    }
}

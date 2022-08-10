#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::executor::Spawner;
use embassy_executor::time::{Duration, Timer};

use embassy_stm32::exti::ExtiInput;
// use embassy_stm32::peripherals::{I2C1, DMA1_CH0, DMA1_CH1};
use embassy_stm32::{
    gpio::{Level, Input, Output, Speed, Pull},
   //  i2c::I2c,
   //  time::Hertz,
    // interrupt,
    peripherals::*,
    Peripherals,
};
use {defmt_rtt as _, panic_probe as _};

// use async_rtc::DS3231;
use async_nucleo::lcd_lib::*;
// use embedded_hal_async::digital::Wait;

#[embassy_executor::main]
async fn main(spawner: Spawner, p: Peripherals) {

    let rs = Output::new(p.PD5, Level::Low, Speed::VeryHigh);
    let rw = Output::new(p.PD4, Level::Low, Speed::VeryHigh);
    let en = Output::new(p.PD3, Level::Low, Speed::VeryHigh);


    let d0 = Output::new(p.PE2, Level::Low, Speed::VeryHigh);
    let d1 = Output::new(p.PE4, Level::Low, Speed::VeryHigh);
    let d2 = Output::new(p.PE5, Level::Low, Speed::VeryHigh);
    let d3 = Output::new(p.PE6, Level::Low, Speed::VeryHigh);

    let d4 = Output::new(p.PE3, Level::Low, Speed::VeryHigh);
    let d5 = Output::new(p.PF8, Level::Low, Speed::VeryHigh);
    let d6 = Output::new(p.PF7, Level::Low, Speed::VeryHigh);
    let d7  =  Input::new(p.PF9, Pull::Down);
    let d7 = ExtiInput::new(d7, p.EXTI9);

    let mut db = DataBus::new(
        rs, rw, en,
        d0, d1, d2, d3,
        d4, d5, d6, d7);


    info!("Wait for Power on");
    Timer::after(Duration::from_millis(40)).await;

    info!("Set Initialize LCD Display");
    // info!("Function set 8-bit");
    db.cmd(0b0011_0000).await.unwrap();
    Timer::after(Duration::from_millis(5)).await;
    // info!("Function set 8-bit");
    db.cmd(0b0011_0000).await.unwrap();
    Timer::after(Duration::from_millis(5)).await;
    // info!("Function set 8-bit");
    db.cmd(0b0011_0000).await.unwrap();

    // info!("rs.set_low(); info!("_instruction_ / data");
    // info!("rw.set_low(); info!("_write_ / read");

    trace!("Function set (8-bit, 2-line, 5x8-font)");
    db.cmd(0b0011_1000).await.unwrap();

    trace!("Display off");
    db.cmd(0b0000_1000).await.unwrap();

    trace!("Clear Display");
    db.cmd(0b0000_0001).await.unwrap();

    trace!("Entry mode set");
    db.cmd(0b0000_0110).await.unwrap();

    info!("End of init");

    trace!("Display control");
    db.cmd(0b0000_1111).await.unwrap();
    // db.write(0b0000_0010).await.unwrap(); info!("Return home");
    // info!("db.write(0b0001_1100).await.unwrap Shift control");

    db.display("Hello, World!").await.unwrap();


    // info!("db.write(0b0011_0000).await.unwrap(); info!("set 8bit mode");

    // Timer::after(Duration::from_micros(100)).await;

    trace!("blinking Cursor");
    db.cmd(0b0000_1111).await.unwrap();

    let _rtc = p.RTC;

    let led = Output::new(p.PB7, Level::High, Speed::Low);

   //  let rtc = DS3231::new(i2c);

    unwrap!(spawner.spawn(blinker(led, Duration::from_micros(500000-213))));
    // unwrap!(spawner.spawn(display(db)));
   //  unwrap!(spawner.spawn(clock(rtc, Duration::from_millis(1000))));
}

#[embassy_executor::task]
async fn blinker(mut led: Output<'static, PB7>, interval: Duration) {
    loop {
        // trace!("blue LED on");
        led.set_high();
        Timer::after(interval).await;

        // trace!("blue LED off");
        led.set_low();
        Timer::after(interval).await;
    }
}

// #[embassy_executor::task]
// async fn display(mut lcd:
//     DataBus<
//     Output<'static, PD5>,
//     Output<'static, PD4>,
//     Output<'static, PD3>,
//     Output<'static, PE2>,
//     Output<'static, PE4>,
//     Output<'static, PE5>,
//     Output<'static, PE6>,
//     Output<'static, PE3>,
//     Output<'static, PF8>,
//     Output<'static, PF7>,
//     ExtiInput<'static, PF9>, // PF9
// >
// ) {
//     loop {
//         // lcd.write(0b0000_1111).await.unwrap();
//         // Timer::after(Duration::from_millis(1000)).await;
//     }
// }

// #[embassy::task] // with more i2c use mutex
// async fn clock(mut rtc: DS3231<I2c<'static, I2C1, DMA1_CH0, DMA1_CH1>>, interval: Duration) {
//     loop {
//         if let Ok(time) = rtc.read_clock().await {

//             info!("clock {:?}", time);
//         }
//         Timer::after(interval).await;
//     }
// }

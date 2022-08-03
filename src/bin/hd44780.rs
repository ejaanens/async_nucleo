#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// // use cortex_m::delay::Delay;
// use defmt::*;
// use embassy::executor::Spawner;
// use embassy::time::{Duration, Timer, Delay};

// use embassy_stm32::rcc::Clocks;
// // use embassy_stm32::peripherals::{I2C1, DMA1_CH0, DMA1_CH1};
// use embassy_stm32::{
//     gpio::{Level, Output, Speed},
//    //  i2c::I2c,
//    //  time::Hertz,
//     // interrupt,
//     peripherals::PB7,
//     Peripherals,
// };
// use {defmt_rtt as _, panic_probe as _};

// // use futures::pin_mut;
// use hd44780_driver::non_blocking::*;

// // use async_rtc::DS3231;

// #[embassy::main]
// async fn main(spawner: Spawner, p: Peripherals) {
//     info!("Hello World!");

//     // let mut delay: Delay = embedded_hal_async::delay::DelayUs;
//     // embassy_stm32::Delay::from(embassy_stm32::time::Timer::new());
//     // embassy_stm32::rcc::

//     // let mut delay = embassy::time::Delay::from(_);
//     // pin_mut!(delay);


//     let mut rs = Output::new(p.PD5, Level::Low, Speed::VeryHigh);
//     let mut rw = Output::new(p.PD4, Level::Low, Speed::VeryHigh);
//     let mut en = Output::new(p.PD3, Level::Low, Speed::VeryHigh);


//     let mut d7 = Output::new(p.PE2, Level::Low, Speed::VeryHigh);
//     let mut d6 = Output::new(p.PE4, Level::Low, Speed::VeryHigh);
//     let mut d5 = Output::new(p.PE5, Level::Low, Speed::VeryHigh);
//     let mut d4 = Output::new(p.PE6, Level::Low, Speed::VeryHigh);
//     let mut d3 = Output::new(p.PE3, Level::Low, Speed::VeryHigh);
//     let mut d2 = Output::new(p.PF8, Level::Low, Speed::VeryHigh);
//     let mut d1 = Output::new(p.PF7, Level::Low, Speed::VeryHigh);
//     let mut d0 = Output::new(p.PF9, Level::Low, Speed::VeryHigh);

//     let mut lcd = HD44780::new_8bit(
//         rs,
//         en,
//         d0,
//         d1,
//         d2,
//         d3,
//         d4,
//         d5,
//         d6,
//         d7,
//     ).await
//     .unwrap();

//     lcd.clear().await;
//     lcd.write_str("Hello, world!").await;



//    //  // Wait for init
//    //  Timer::after(Duration::from_millis(15)).await;

//    //  // Set 8-bit mode
//    //  rs.set_low(); // _instruction_ / data
//    //  rw.set_low(); // _write_ / read

//    //  Timer::after(Duration::from_micros(1)).await;

//    //  db7.set_low();
//    //  db6.set_low();
//    //  db5.set_high();
//    //  db4.set_high();

//    //  en.set_high(); // enable

//    //  Timer::after(Duration::from_micros(1)).await;

//    //  en.set_low(); // disable

//    //  Timer::after(Duration::from_millis(5)).await;

//    //  //





//    //  Timer::after(Duration::from_micros(1)).await;

//    //  db7.set_low();
//    //  db6.set_low();
//    //  db5.set_low();
//    //  db4.set_low();

//    //  db3.set_high();
//    //  db2.set_high();
//    //  db1.set_high();
//    //  db0.set_high();

//    //  en.set_high(); // enable

//    //  Timer::after(Duration::from_micros(1)).await;

//    //  en.set_low(); // disable




//     // let db = [
//     // ];


//    //  let irq = p.I2C1.into();

//    //  let i2c = I2c::new(
//    //      p.I2C1,
//    //      p.PB8,
//    //      p.PB9,
//    //      irq,
//    //      p.DMA1_CH0,
//    //      p.DMA1_CH1,
//    //      Hertz::khz(400),
//    //  );

//    //  p.P

//     let led = Output::new(p.PB7, Level::High, Speed::Low);

//    //  let rtc = DS3231::new(i2c);

//     unwrap!(spawner.spawn(blinker(led, Duration::from_micros(500000-213))));
//    //  unwrap!(spawner.spawn(clock(rtc, Duration::from_millis(1000))));
// }

// #[embassy::task]
// async fn blinker(mut led: Output<'static, PB7>, interval: Duration) {
//     loop {
//         info!("high");
//         led.set_high();
//         Timer::after(interval).await;

//         info!("low");
//         led.set_low();
//         Timer::after(interval).await;
//     }
// }

// // #[embassy::task] // with more i2c use mutex
// // async fn clock(mut rtc: DS3231<I2c<'static, I2C1, DMA1_CH0, DMA1_CH1>>, interval: Duration) {
// //     loop {
// //         if let Ok(time) = rtc.read_clock().await {

// //             info!("clock {:?}", time);
// //         }
// //         Timer::after(interval).await;
// //     }
// // }

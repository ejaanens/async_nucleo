
// #![no_std]

use embedded_hal_async::i2c::I2c;

const RTC: u8 = 0x68;

const CLOCK: u8 = 0x00;

#[derive(Debug)]
pub enum Freq {
    Sqw1Hz    = 0b00,
    Sqw1024Hz = 0b01,
    Sqw4096Hz = 0b10,
    Sqw8192Hz = 0b11,
}

/// Clock struct hour, min, sec.
#[derive(Debug)]
pub struct Clock {
    /// seconds 0-59
    pub sec: u8,
    /// minutes 0-59
    pub min: u8,
    /// hours 1-12am/pm or 0-23
    pub hour: u8, // Hour,
}

#[derive(Debug)]
pub struct DS3231<I2C> {
    i2c: I2C,
}

impl <I2C, E> DS3231<I2C>
where
    I2C: I2c<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(i2c: I2C) -> Self {
        Self {i2c}
    }

    // async fn write_reg(&mut self, reg: u8, val: u8) -> Result<(), E> {
    //     let buf = [reg, val];

    //     self.i2c.write(RTC, &buf).await?;
    //     Ok(())
    // }

    // pub async fn set_clock(&mut self, time: u8) {

    // }

    pub async fn read_clock(&mut self) -> Result<Clock, E> {
        let mut buf = [CLOCK, 3];

        self.i2c.read(RTC, &mut buf).await?;

        Ok(Clock {
            sec: buf[0],
            min: buf[1],
            hour: buf[2],
        })
    }
}

/*!
Rust async real time clock driver for DS3231 based on [`embedded-hal-async`] traits.
*/

#![no_std]
#![deny(unsafe_code, missing_debug_implementations)]

extern crate embedded_hal_async;

use embedded_hal_async::i2c::*;
use nb::Result;

const ADDR : u8 = 0x68;
const CLOCK: u8 = 0x00;
const DATE : u8 = 0x04;
const CTRL : u8 = 0x0E;

/// RTC I²C interface.
#[derive(Debug)]
pub struct Rtc<I2c> {
    i2c: I2c,
}

// WriteFuture

impl <I2c, E> Rtc<I2c>
where
    I2c: WriteFuture<SevenBitAddress, Error = E>,
{
    /// Sets the clock.
    pub fn set_clock(&mut self, hour: u8, min: u8, sec: u8) -> Result<(), E>{
        let bcd_hour = bin_to_bcd(hour);
        let bcd_min  = bin_to_bcd(min);
        let bcd_sec  = bin_to_bcd(sec);

        let bytes = [0x00, bcd_sec, bcd_min, bcd_hour];

        self.i2c.write(ADDR, &bytes)
    }

    /// Sets the date.
    pub fn set_date(&mut self, century: u8, year: u8, month: u8, date: u8) -> Result<(), E> {
        let     bcd_year  = bin_to_bcd(year);
        let mut bcd_month = bin_to_bcd(month);
        let     bcd_date  = bin_to_bcd(date);

        bcd_month |= century << 7;

        let bytes = [DATE, bcd_date, bcd_month, bcd_year];

        self.i2c.write(ADDR, &bytes)
    }

    pub fn enable_sqw(&mut self, freq: Freq) -> Result<(), E> {

        let rs: u8 = match freq {
            Freq::Hz1    => 0b00 << 3,
            Freq::Hz1024 => 0b01 << 3,
            Freq::Hz4096 => 0b10 << 3,
            Freq::Hz8192 => 0b11 << 3,
        };

        self.i2c.write(ADDR, &[CTRL, rs])?;
        Ok(())
    }

}

impl <I2C, E> Rtc<I2C>
where
    I2C: WriteReadFuture<SevenBitAddress, Error = E>,
{
    /// Read the clock.
    pub fn read_clock(&mut self) -> Result<Clock, E> {
        let mut buf = [0x00, 3];
        self.i2c.write_read(ADDR, &[CLOCK], &mut buf)?;
        let hour = if buf[2] & 0b0100_0000 == 64 {
            // 12
            let mdm = if (buf[2] & 0b0010_0000) == 0 {
                Meridiem::AM
            } else {
                Meridiem::PM
            };
            Hour::H12((bcd_to_bin(buf[2] & 0b0001_1111), mdm))
        } else {
            //24
            Hour::H24(bcd_to_bin(buf[2] & 0b0011_1111))

        };

        Ok(Clock {
            sec: bcd_to_bin(buf[0]),
            min: bcd_to_bin(buf[1]),
            hour,
        })
    }

    /// Reads the date year/month/day.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// Ok(Date {24, 7, 2022}) = rtc.read_date();
    /// ```
    pub fn read_date(&mut self) -> Result<Date, E> {
        let mut buf = [0x00; 3];
        self.i2c.write_read(ADDR, &[DATE], &mut buf)?;
        let century = (buf[1] >> 7) * 100;
        let century: u16 = (bcd_to_bin(buf[2]) + century).into();
        let year: u16 = 2000_u16 + century;
        Ok(Date {
            date: bcd_to_bin(buf[0]),
            month: bcd_to_bin(buf[1] & 0x1F),
            year,
        })
    }
}

#[inline]
fn bcd_to_bin(bcd: Bcd) -> u8 {
    (bcd >> 4) * 10 + (bcd & 0xF)
}

#[inline]
fn bin_to_bcd(bin: u8) -> Bcd {
    ((bin / 10) << 4) | (bin % 10)
}


type Bcd = u8;

#[derive(Debug)]
pub enum Hour {
    H12((u8, Meridiem)),
    H24(u8),
}

#[derive(Debug)]
pub enum Meridiem {
    AM,
    PM,
}

#[derive(Debug)]
pub enum Freq {
    Hz1,
    Hz1024,
    Hz4096,
    Hz8192,
}

/// Clock struct hour, min, sec.
#[derive(Debug)]
pub struct Clock {
    /// seconds 0-59
    pub sec: u8,
    /// minutes 0-59
    pub min: u8,
    /// hours 1-12am/pm or 0-23
    pub hour: Hour,
}
// day: u8,

/// Date struct day, month year.
#[derive(Debug)]
pub struct Date {
    /// day in month
    pub date: u8,
    /// month #
    pub month: u8,
    /// year.
    pub year: u16,
    // pub century: u8,

}

// pub struct Alarm1 {
//    sec: BCD,
//    min: BCD,
//    hour: BCD24,
//    day: u8,
//    date: BCD,
// }

// pub struct Alarm2 {
//    min: BCD,
//    hour: BCD24,
//    day: u8,
//    date: BCD,
// }

// impl I2C for RTC {

// }


#[cfg(test)]
mod tests {

}

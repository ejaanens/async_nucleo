use embedded_hal_async::i2c::I2c;
use rtcc::*;
use nb;
use chrono::Weekday;

const ADDR : u8 = 0x68;

#[derive(Copy, Clone, Debug)]
pub enum Error<ComErr> {
    /// I²C communication Error
    I2c(ComErr),
}

pub enum Reg {
    SEC     = 0x00,
    MIN     = 0x01,
    HOUR    = 0x02,
    DAY     = 0x03,
    DATE    = 0x04,
    MONTH   = 0x05,
    YEAR    = 0x06,
    ALRM1   = 0x07,
    ALRM2   = 0x0B,
    CTRL    = 0x0E,
    STAT    = 0x0F,
    AGOFS   = 0x10,
    TEMP    = 0x11,
}

#[derive(Debug)]
pub enum Freq {
    Sqw1Hz    = 0b00,
    Sqw1024Hz = 0b01,
    Sqw4096Hz = 0b10,
    Sqw8192Hz = 0b11,
}

fn bin_to_bcd(bin: u8) -> u8 {
    ((bin / 10) << 4) | (bin % 10)
}

fn bcd_to_bin(bcd: u8) -> u8 {
    (bcd >> 4) * 10 + (bcd & 0xF)
}

fn to_hours(hour: u8) -> Hours {
    if hour & (1 << 6) != 0 {
        if hour & (1 << 5) != 0 {
            Hours::PM(bcd_to_bin(hour & 0b0001_1111))
        } else {
            Hours::AM(bcd_to_bin(hour & 0b0001_1111))
        }
    } else {
        Hours::H24(bcd_to_bin(hour & 0b0011_1111))
    }
}

#[derive(Debug)]
pub struct DS3231<I2C> {
    i2c: I2C,
}

impl <I2C, ComErr> DS3231<I2C>
where
    I2C: I2c<Error = ComErr>,
    ComErr: core::fmt::Debug,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub async fn init(&mut self) -> nb::Result<(), Error<ComErr>> {
        let write = [Reg::STAT as u8, 0x00]; // Clear status register

        self.i2c.write(ADDR, &write).await.map_err(Error::I2c)?;

        Ok(())
    }

    pub async fn osc_on_bat(&mut self, disable: bool) -> nb::Result<(), Error<ComErr>> {
        let mut buf = [Reg::CTRL as u8];

        self.i2c.read(ADDR, &mut buf).await.map_err(Error::I2c)?;
        if disable {
            buf[0] |= 1 << 7;
        } else {
            buf[0] &= !(1 << 7);
        }
        self.i2c.write(ADDR, &mut buf).await.map_err(Error::I2c)?;
        Ok(())
    }

    pub async fn set_sqw_frq(&mut self, freq: Freq) -> nb::Result<(), Error<ComErr>> {
        let mut read = [Reg::CTRL as u8];

        let freq = match freq {
            Freq::Sqw1Hz    => 0b00 << 3,
            Freq::Sqw1024Hz => 0b01 << 3,
            Freq::Sqw4096Hz => 0b10 << 3,
            Freq::Sqw8192Hz => 0b11 << 3,
        };
        self.i2c.read(ADDR, &mut read).await.map_err(Error::I2c)?;

        read[0] &= 0b1110_0011; // clear freq and int ctrl
        read[0] |= (1 << 6) | freq; // enable squarewave & set frequency

        let write = [Reg::CTRL as u8, read[0]];

        self.i2c.write(ADDR, &write).await.map_err(Error::I2c)?;
        Ok(())
    }

    pub async fn Output_32kHz(&mut self, disable: bool) -> nb::Result<(), Error<ComErr>> {
        let mut read = [Reg::STAT as u8];

        self.i2c.read(ADDR, &mut read).await.map_err(Error::I2c)?;

        let en32khz = read[0] & 0b0000_1000 != 0;

        if en32khz == disable {
            return Ok(());
        }
        read[0] &= 0b1111_0111;

        let stat = if disable {
            0b0000_0000 | read[0]
        } else {
            0b0000_1000 | read[0]
        };
        let write = [Reg::STAT as u8, stat];

        self.i2c.write(ADDR, &write).await.map_err(Error::I2c)?;
        Ok(())
    }

    async fn read_bcd_reg(&mut self, reg: Reg) -> nb::Result<u8, Error<ComErr>> {
        let mut buf = [reg as u8];

        self.i2c.read(ADDR, &mut buf).await.map_err(Error::I2c)?;

        Ok(bcd_to_bin(buf[0]))
    }

    async fn set_bcd_reg(&mut self, reg: Reg, value: u8) -> nb::Result<(), Error<ComErr>> {
        let mut buf = [reg as u8, bin_to_bcd(value)];

        self.i2c.write(ADDR, &mut buf).await.map_err(Error::I2c)?;

        Ok(())
    }


}

impl <I2C, ComErr> DS3231<I2C>
where
    I2C: I2c<Error = ComErr>,
    ComErr: core::fmt::Debug,
{
    type Error = Error<ComErr>;

    pub async fn datetime(&mut self) -> nb::Result<NaiveDateTime, Error<ComErr>> {

        let mut buf = [Reg::SEC as u8; 7];

        self.i2c.read(ADDR, &mut buf).await.map_err(Error::I2c)?;

        let year  = bcd_to_bin(buf[6]).into();
        let month = bcd_to_bin(buf[5]).into();
        let day   = bcd_to_bin(buf[4]).into();
        // let week day = buf[3];
        let hour = match to_hours(buf[2]) {
            Hours::H24(hour) => hour.into(),
            Hours::AM(hour) => hour.into(),
            Hours::PM(hour) => hour.into(),
        };
        let min = bcd_to_bin(buf[1]).into();
        let sec = bcd_to_bin(buf[0]).into();

        Ok(NaiveDate::from_ymd(year, month, day).and_hms(hour, min, sec))
    }

    pub async fn set_datetime(&mut self, datetime: &NaiveDateTime)
        -> nb::Result<(), Error<ComErr>>
    {
        let sec = bin_to_bcd(datetime.second() as u8);
        let min = bin_to_bcd(datetime.minute() as u8);
        let hour = bin_to_bcd(datetime.hour() as u8); // .hour12()

        let day: u8 = match datetime.weekday() {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 7,
        };

        let date  = bin_to_bcd(datetime.day()   as u8);
        let month = bin_to_bcd(datetime.month() as u8); // TODO century
        let year  = bin_to_bcd(datetime.year()  as u8);

        // let test = datetime.year() - 2000 - year;

        let buf = [Reg::SEC as u8, sec, min, hour, day, date, month, year];

        self.i2c.write(ADDR, &buf).await.map_err(Error::I2c)?;

        Ok(())
    }

    pub async fn seconds(&mut self) -> nb::Result<u8, Error<ComErr>> {
        self.read_bcd_reg(Reg::SEC).await
    }

    pub async fn minutes(&mut self) -> nb::Result<u8, Error<ComErr>> {
        self.read_bcd_reg(Reg::MIN).await
    }

    pub async fn hours(&mut self) -> nb::Result<Hours, Error<ComErr>> {
        let mut buf = [Reg::HOUR as u8];
        self.i2c.read(ADDR, &mut buf).await.map_err(Error::I2c)?;

        Ok(to_hours(buf[0]))
    }

    pub async fn time(&mut self) -> nb::Result<NaiveTime, Error<ComErr>> {
        let mut buf = [Reg::SEC as u8; 3];
        self.i2c.read(ADDR, &mut buf).await.map_err(Error::I2c)?;
        let sec = bcd_to_bin(buf[0]);
        let min = bcd_to_bin(buf[1]);
        let hours = to_hours(buf[2]);

        let hour = match hours {
            Hours::H24(hour) => hour as u32,
            Hours::AM(hour) => hour as u32,
            Hours::PM(hour) => (hour + 12u8) as u32,
        };

        Ok(NaiveTime::from_hms(hour, min as u32, sec as u32))
    }

    pub async fn weekday(&mut self) -> nb::Result<u8, Error<ComErr>> {
        let mut buf = [Reg::DAY as u8];
        self.i2c.read(ADDR, &mut buf).await.map_err(Error::I2c)?;
        Ok(buf[0])
    }

    pub async fn day(&mut self) -> nb::Result<u8, Error<ComErr>> {
        self.read_bcd_reg(Reg::DATE).await
    }

    pub async fn month(&mut self) -> nb::Result<u8, Error<ComErr>> {
        let mut buf = [Reg::MONTH as u8];
        self.i2c.read(ADDR, &mut buf).await.map_err(Error::I2c)?;
        Ok(bcd_to_bin(buf[0] &  0b0111_1111))
    }

    pub async fn year(&mut self) -> nb::Result<u16, Error<ComErr>> {
        let mut buf = [Reg::MONTH as u8; 2];
        self.i2c.read(ADDR, &mut buf).await.map_err(Error::I2c)?;
        let cent = (((buf[0] & (1 << 7)) >> 7) * 100) as u16;
        let year = bcd_to_bin(buf[1]) as u16;
        Ok(2000u16 + cent + year)
    }

    pub async fn date(&mut self) -> nb::Result<NaiveDate, Error<ComErr>> {
        let mut buf = [Reg::DATE as u8; 3];
        self.i2c.read(ADDR, &mut buf).await.map_err(Error::I2c)?;

        let day = bcd_to_bin(buf[0]);
        let month = bcd_to_bin(buf[1] & 0b0111_1111);

        let cent = (((buf[0] & (1 << 7)) >> 7) * 100) as u16;
        let year = bcd_to_bin(buf[1]) as u16;
        let year = 2000u16 + cent + year;

        Ok(NaiveDate::from_ymd(year as i32, month as u32, day as u32))
    }

    pub async fn set_seconds(&mut self, seconds: u8) -> nb::Result<(), Error<ComErr>> {
        self.set_bcd_reg(Reg::SEC, seconds).await
    }

    pub async fn set_minutes(&mut self, minutes: u8) -> nb::Result<(), Error<ComErr>> {
        self.set_bcd_reg(Reg::MIN, minutes).await
    }

    pub async fn set_hours(&mut self, hours: Hours) -> nb::Result<(), Error<ComErr>> {
        let hour = match hours {
            Hours::H24(hour) => hour,
            Hours::AM(hour) => hour,
            Hours::PM(hour) => hour,
        };
        let hour = bin_to_bcd(hour);
        let buf = [Reg::HOUR as u8, hour];

        self.i2c.write(ADDR, &buf).await.map_err(Error::I2c)?;
        Ok(())
    }

    pub async fn set_time(&mut self, time: &NaiveTime) -> nb::Result<(), Error<ComErr>> {
        let buf = [
            Reg::SEC as u8,
            time.second() as u8,
            time.minute() as u8,
            time.hour() as u8
        ];

        self.i2c.write(ADDR, &buf).await.map_err(Error::I2c)?;
        Ok(())
    }

    pub async fn set_weekday(&mut self, weekday: u8) -> nb::Result<(), Error<ComErr>> {
        self.set_bcd_reg(Reg::DAY, weekday).await
    }

    pub async fn set_day(&mut self, day: u8) -> nb::Result<(), Error<ComErr>> {
        self.set_bcd_reg(Reg::DATE, day).await
    }

    pub async fn set_month(&mut self, month: u8) -> nb::Result<(), Error<ComErr>> {
        self.set_bcd_reg(Reg::MONTH, month).await
    }

    pub async fn set_year(&mut self, year: u16) -> nb::Result<(), Error<ComErr>> {
        self.set_bcd_reg(Reg::YEAR, year as u8).await
    }

    pub async fn set_date(&mut self, date: &NaiveDate) -> nb::Result<(), Error<ComErr>> {
        let day   = bin_to_bcd(date.day() as u8);
        let month = bin_to_bcd(date.month() as u8);
        let year  = bin_to_bcd(date.year() as u8);

        let buf = [Reg::DATE as u8, day, month, year];

        self.i2c.write(ADDR, &buf).await.map_err(Error::I2c)?;
        Ok(())
    }
}

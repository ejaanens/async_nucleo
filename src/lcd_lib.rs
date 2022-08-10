use defmt::{trace, info};
use embassy_executor::time::{Duration, Timer};
// use embedded_hal_async::{self, digital::Wait};
use embedded_hal::digital::v2::OutputPin; // {IoPin, ,}
use embassy_stm32::{exti::ExtiInput, peripherals::PF9};

// use core::arch::asm;

#[derive(Debug)]
pub struct Error;
pub type Result<T> = core::result::Result<T, Error>;

// match
// カ => 0b1011_0110
// 。


//  ータミαp
// 。アチムäq
//  イツメßθ
//  ウテモε
// 、エトヤµ Ω
// ・オナユσ ü
//  カニヨρ Σ
//  フキヌラg π
//  クネリsqx
//  ケノル-1y
//  コハレj
//  サヒロx
//  シフワ¢
//  スヘソ
//  セホ
//  ソマ ö

// 四
// 一ニ

// ガギゲグゴ
// パピプペポ


// ￥
pub struct DataBus<
    RS: OutputPin,
    RW: OutputPin,
    EN: OutputPin,
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    // D7: Wait,
> {
    rs: RS,
    rw: RW,
    en: EN,
    d0: D0,
    d1: D1,
    d2: D2,
    d3: D3,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: ExtiInput<'static, PF9>// D7,
}

impl<
    RS: OutputPin, // IoPin<InputPin, _>
    RW: OutputPin, // IoPin<InputPin, _>
    EN: OutputPin, // IoPin<InputPin, _>
    D0: OutputPin, // IoPin<InputPin, _>
    D1: OutputPin, // IoPin<InputPin, _>
    D2: OutputPin, // IoPin<InputPin, _>
    D3: OutputPin, // IoPin<InputPin, _>
    D4: OutputPin, // IoPin<InputPin, _>
    D5: OutputPin, // IoPin<InputPin, _>
    D6: OutputPin, // IoPin<InputPin, _>
    // D7: Wait, // IoPin<InputPin, _>
    > DataBus<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6> //, D7>
    {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        rs: RS,
        rw: RW,
        en: EN,
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: ExtiInput<'static, PF9> // D7
    ) -> Self {
        Self {
            rs,
            rw,
            en,
            d0,
            d1,
            d2,
            d3,
            d4,
            d5,
            d6,
            d7,
        }
    }

    fn set_bus_bits(&mut self, data: u8) -> Result<()>{
        // let _db7: bool = (0b1000_0000 & data) != 0;
        let db6: bool = (0b0100_0000 & data) != 0;
        let db5: bool = (0b0010_0000 & data) != 0;
        let db4: bool = (0b0001_0000 & data) != 0;
        let db3: bool = (0b0000_1000 & data) != 0;
        let db2: bool = (0b0000_0100 & data) != 0;
        let db1: bool = (0b0000_0010 & data) != 0;
        let db0: bool = (0b0000_0001 & data) != 0;

        // if db7 { self.d7.set_high().map_err(|_| Error)?;}
        // else   { self.d7.set_low().map_err(|_| Error)?;}
        if db6 { self.d6.set_high().map_err(|_| Error)?;}
        else   { self.d6.set_low().map_err(|_| Error)?;}
        if db5 { self.d5.set_high().map_err(|_| Error)?;}
        else   { self.d5.set_low().map_err(|_| Error)?;}
        if db4 { self.d4.set_high().map_err(|_| Error)?;}
        else   { self.d4.set_low().map_err(|_| Error)?;}
        if db3 { self.d3.set_high().map_err(|_| Error)?;}
        else   { self.d3.set_low().map_err(|_| Error)?;}
        if db2 { self.d2.set_high().map_err(|_| Error)?;}
        else   { self.d2.set_low().map_err(|_| Error)?;}
        if db1 { self.d1.set_high().map_err(|_| Error)?;}
        else   { self.d1.set_low().map_err(|_| Error)?;}
        if db0 { self.d0.set_high().map_err(|_| Error)?;}
        else   { self.d0.set_low().map_err(|_| Error)?;}

        Ok(())
    }

    // async fn read(&mut self) -> Result<()> {
    // }

    async fn write(&mut self, byte: u8) -> Result<()> {
        // Wait for Busy Flag off
        self.rw.set_high().map_err(|_| Error)?; // read mode
        self.en.set_high().map_err(|_| Error)?;
        self.d7.wait_for_low().await; //.map_err(|_| Error)?; // check busy flag
        self.en.set_low().map_err(|_| Error)?;

        // Write data
        self.rw.set_low().map_err(|_| Error)?;
        self.set_bus_bits(byte).map_err(|_| Error)?;
        trace!("Writing: {:08b}", byte);
        self.en.set_high().map_err(|_| Error)?;
        Timer::after(Duration::from_millis(2)).await;
        trace!("Done!");
        self.en.set_low().map_err(|_| Error)?;
        Timer::after(Duration::from_millis(2)).await;
        Ok(())
    }

    pub async fn cmd(&mut self, cmd: u8) -> Result<()> {
        self.rs.set_low().map_err(|_| Error)?;
        self.write(cmd).await?;
        Ok(())
    }

    pub async fn display(&mut self, string: &str) -> Result<()>{
        self.rs.set_high().map_err(|_| Error)?;
        let bytes = string.as_bytes();
        for &byte in bytes {
            self.write(byte).await?;
        }
        Ok(())
    }
}

// }

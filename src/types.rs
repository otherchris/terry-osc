use crate::samples::Sample;
use crate::MicrosDurationU32;
use crate::{pac::I2C0, pac::I2C1, Alarm0, Alarm1, Alarm2, Alarm3, I2C};
use crate::{RotaryEncoder, StandardMode};
use mcp4725::MCP4725;
use rp2040_hal::{
    gpio::bank0::{
        Gpio10, Gpio11, Gpio12, Gpio13, Gpio14, Gpio15, Gpio18, Gpio19, Gpio20, Gpio21, Gpio8,
        Gpio9,
    },
    gpio::{FunctionI2c, FunctionSioInput, FunctionUart, Pin, PullDown, PullUp},
    pac::UART1,
    uart::{Enabled, UartPeripheral},
};
use ssd1306::mode::TerminalMode;
use ssd1306::prelude::I2CInterface;
use ssd1306::size::DisplaySize128x32;
use ssd1306::Ssd1306;

pub type SdaPin = Pin<Gpio18, FunctionI2c, PullDown>;
pub type SclPin = Pin<Gpio19, FunctionI2c, PullDown>;
pub type I2C1Type = I2C<I2C1, (SdaPin, SclPin)>;

pub type DacType = MCP4725<I2C1Type>;

pub type DisplaySdaPin = Pin<Gpio20, FunctionI2c, PullDown>;
pub type DisplaySclPin = Pin<Gpio21, FunctionI2c, PullDown>;
pub type DisplayI2c = I2C<I2C0, (DisplaySdaPin, DisplaySclPin)>;

pub type TxPin = rp2040_hal::gpio::Pin<Gpio8, FunctionUart, PullDown>;
pub type RxPin = rp2040_hal::gpio::Pin<Gpio9, FunctionUart, PullDown>;
pub type Uart1Type = UartPeripheral<Enabled, UART1, (TxPin, RxPin)>;

pub type RotaryEncoderType = RotaryEncoder<
    StandardMode,
    Pin<Gpio15, FunctionSioInput, PullUp>,
    Pin<Gpio14, FunctionSioInput, PullUp>,
>;

pub type RotaryEncoderButton = Pin<Gpio11, FunctionSioInput, PullUp>;
pub type RotaryEncoderDt = Pin<Gpio15, FunctionSioInput, PullUp>;

pub struct ModuleState<'a> {
    pub encoder: RotaryEncoderType,
    pub encoder_button: RotaryEncoderButton,
    pub display: Ssd1306<I2CInterface<DisplayI2c>, DisplaySize128x32, TerminalMode>,
    pub sample: &'a mut Sample,
    pub sample_length: usize,
}

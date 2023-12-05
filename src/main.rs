//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]
use core::cell::RefCell;
use critical_section::Mutex;
use defmt::*;
use defmt_rtt as _;
use fugit::{MicrosDurationU32, RateExtU32};
use mcp4725::*;
use panic_probe as _;
use rp2040_hal::{
    clocks::init_clocks_and_plls,
    gpio::Pins,
    pac,
    pac::interrupt,
    timer::{Alarm0, Alarm1, Alarm2, Alarm3, Timer},
    watchdog::Watchdog,
    Clock, Sio, I2C,
};
use rp_pico::entry;
mod types;
mod uart_interrupt;
use rotary_encoder_embedded::{standard::StandardMode, RotaryEncoder};
use types::ModuleState;

static mut MODULE_STATE: Mutex<RefCell<Option<ModuleState>>> = Mutex::new(RefCell::new(None));
static INITIAL_ALARM_DURATION: MicrosDurationU32 = MicrosDurationU32::micros(100000);
static INITIAL_ENCODER_POLL_DURATION: MicrosDurationU32 = MicrosDurationU32::micros(2000);

#[entry]
fn main() -> ! {
    info!("Program start");
    let pac = pac::Peripherals::take().unwrap();
    let mut resets = pac.RESETS;
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);
    let external_xtal_freq_hz = 12_000_000u32;
    info!("Setting clocks");
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut resets,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    info!("creating timer");
    let mut timer = Timer::new(pac.TIMER, &mut resets, &clocks);

    let pins = Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut resets);

    info!("set up i2c0");
    let scl = pins.gpio19.into_function();
    let sda = pins.gpio18.into_function();
    let i2c1_device =
        I2C::new_controller(pac.I2C1, sda, scl, 400.kHz(), &mut resets, 125_000_000.Hz());

    info!("create dac");
    let mut dac = MCP4725::new(i2c1_device, 0b000);
    dac.set_dac(PowerDown::Normal, 0xfff);
    let mut val1 = 0;
    let mut val2 = 0;
    let mut val3 = 0;
    let mut on = true;
    loop {
        if val1 >= 0xfff {
            val1 = 0;
        }
        if val2 >= 0x556 {
            val2 = 0;
        }
        if val3 >= 0xfff {
            val3 = 0;
        }
        // val1 += 50;
        // val2 += 72;
        val3 += 113;
        // info!("{}", val);
        dac.set_dac_fast(PowerDown::Normal, val1 + val2 + val3);
    }
}

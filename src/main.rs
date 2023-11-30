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
    timer::{Alarm, Alarm0, Alarm1, Alarm2, Alarm3, Timer},
    uart::{DataBits, StopBits, UartConfig, UartPeripheral},
    watchdog::Watchdog,
    Clock, Sio, I2C,
};
use rp_pico::entry;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
mod io_irq_bank0;
mod samples;
mod types;
use rotary_encoder_embedded::{standard::StandardMode, RotaryEncoder};
use samples::{advance_osc, new_osc, ramp, Osc, OscState, Wave};
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
    let mut dac = MCP4725::new(i2c1_device, 0b010);
    dac.set_dac(PowerDown::Normal, 0x0);

    info!("set up i2c1");
    let display_scl = pins.gpio21.into_function();
    let display_sda = pins.gpio20.into_function();
    let i2c0_device = I2C::new_controller(
        pac.I2C0,
        display_sda,
        display_scl,
        400.kHz(),
        &mut resets,
        125_000_000.Hz(),
    );
    info!("set up display");
    let interface = I2CDisplayInterface::new(i2c0_device);
    let mut display =
        Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0).into_terminal_mode();
    display.init().unwrap();
    display.clear().ok();

    info!("set up encoder");
    let encoder_dt = pins.gpio15.into_pull_up_input();
    encoder_dt.set_interrupt_enabled(rp2040_hal::gpio::Interrupt::EdgeLow, true);
    let encoder_clk = pins.gpio14.into_pull_up_input();
    let encoder = RotaryEncoder::new(encoder_dt, encoder_clk).into_standard_mode();
    let encoder_button = pins.gpio11.reconfigure();
    encoder_button.set_interrupt_enabled(rp2040_hal::gpio::Interrupt::EdgeHigh, true);

    let mut sample = [Some(0); 1000];
    ramp(100, 0xfff, &mut sample);
    critical_section::with(|cs| {
        info!("Create module state");
        unsafe {
            MODULE_STATE.borrow(cs).replace(Some(ModuleState {
                encoder,
                encoder_button,
                display,
                sample: &mut sample,
                sample_length: 100,
            }));
        }
        // Don't unmask the interrupts until the Module State is in place
        info!("Unmask interrupts");
        unsafe {
            // pac::NVIC::unmask(pac::Interrupt::TIMER_IRQ_0);
            // pac::NVIC::unmask(pac::Interrupt::TIMER_IRQ_1);
            // pac::NVIC::unmask(pac::Interrupt::TIMER_IRQ_2);
            // pac::NVIC::unmask(pac::Interrupt::TIMER_IRQ_3);
            // pac::NVIC::unmask(pac::Interrupt::UART1_IRQ);
            pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0)
        }
        info!("done!");
    });

    info!("looping");
    let mut i = 0;
    let mut val = 0;
    loop {
        // osc_state.osc_1 = reset_osc(osc_state.osc_1);
        // osc_state.osc_1 = reset_osc(osc_state.osc_1);
        (i, val) = match sample[i] {
            Some(val) => (i + 1, val),
            None => (0, val),
        };
        // let (val3, osc_3) = advance_osc(osc_state.osc_3);
        // let (val4, osc_4) = advance_osc(osc_state.osc_4);

        dac.set_dac_fast(PowerDown::Normal, val);
    }
}

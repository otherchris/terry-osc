use crate::{info, interrupt, pac, ramp, samples::Osc, ModuleState, Pins, Sio, MODULE_STATE};
use rotary_encoder_embedded::Direction;
use rp2040_hal::gpio::Interrupt::{EdgeHigh, EdgeLow};

#[interrupt]
fn IO_IRQ_BANK0() {
    info!("irqed");
    critical_section::with(|cs| {
        let module_state = unsafe { MODULE_STATE.borrow(cs).take().unwrap() };
        let ModuleState {
            mut encoder_button,
            mut change,
            mut encoder,
            ..
        } = module_state;
        if encoder_button.interrupt_status(EdgeHigh) {
            change = true;
            encoder_button.clear_interrupt(EdgeHigh);
        }
        let (dt, clk) = encoder.pins_mut();
        if dt.interrupt_status(EdgeLow) {
            info!("LOWW");
            dt.clear_interrupt(EdgeLow);
            encoder.update();
            match encoder.direction() {
                Direction::Clockwise => {
                    info!("clocky");
                    change = true
                }
                Direction::Anticlockwise => {
                    info!("anticlocky");
                    change = true
                }
                Direction::None => {
                    // info!("None")
                }
            }
        }
        //         let (dt, _) = encoder.pins_mut();
        //         dt.clear_interrupt(EdgeLow);
        unsafe {
            MODULE_STATE.borrow(cs).replace(Some(ModuleState {
                encoder,
                encoder_button,
                change,
                ..module_state
            }))
        }
    });
    // }
}

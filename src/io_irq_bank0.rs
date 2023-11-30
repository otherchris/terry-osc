use crate::{info, interrupt, pac, ramp, samples::Osc, ModuleState, Pins, Sio, MODULE_STATE};
use rotary_encoder_embedded::Direction;
use rp2040_hal::gpio::Interrupt::{EdgeHigh, EdgeLow};

#[interrupt]
fn IO_IRQ_BANK0() {
    info!("irqed");
    critical_section::with(|cs| {
        let module_state = unsafe { MODULE_STATE.borrow(cs).take().unwrap() };
        let ModuleState {
            mut encoder,
            mut encoder_button,
            mut display,
            sample,
            mut sample_length,
            ..
        } = module_state;
        if encoder_button.interrupt_status(EdgeHigh) {
            display.clear().ok();
            for c in ['b', 'u', 't', 't', '1'] {
                display.print_char(c).ok();
            }
            encoder_button.clear_interrupt(EdgeHigh);
        }
        encoder.update();
        match encoder.direction() {
            Direction::Clockwise => {
                info!("clocky");
                ramp(sample_length + 1, 0xfff, sample);
                sample_length += 1;
            }
            Direction::Anticlockwise => {
                info!("anticlocky");
                ramp(sample_length - 1, 0xfff, sample);
                sample_length -= 1;
            }
            Direction::None => {
                // info!("None")
            }
        }
        let (dt, _) = encoder.pins_mut();
        dt.clear_interrupt(EdgeLow);
        unsafe {
            MODULE_STATE.borrow(cs).replace(Some(ModuleState {
                encoder,
                encoder_button,
                display,
                ..module_state
            }))
        }
    });
}

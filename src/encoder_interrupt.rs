use rotary_encoder_embedded::Direction;
use rp2040_hal::timer::Alarm;

use crate::{info, interrupt, pac, samples::Osc, ModuleState, Pins, Sio, MODULE_STATE};

#[interrupt]
fn TIMER_IRQ_3() {
    critical_section::with(|cs| {
        let module_state = unsafe { MODULE_STATE.borrow(cs).take().unwrap() };
        let ModuleState {
            mut encoder_poll_duration,
            mut encoder_poll_alarm,
            mut encoder_1,
            mut encoder_2,
            mut display,
            mut osc_state,
            ..
        } = module_state;
        encoder_poll_alarm.clear_interrupt();
        encoder_poll_alarm.schedule(encoder_poll_duration).ok();

        encoder_1.update();
        encoder_2.update();
        match encoder_1.direction() {
            Direction::Clockwise => {
                info!("clocky");
                osc_state.osc_1 = Osc {
                    sample_count: osc_state.osc_1.sample_count - 1,
                    edit_flag: true,
                    ..osc_state.osc_1
                };
            }
            Direction::Anticlockwise => {
                info!("anticlocky");
                osc_state.osc_1 = Osc {
                    sample_count: osc_state.osc_1.sample_count + 1,
                    edit_flag: true,
                    ..osc_state.osc_1
                };
            }
            Direction::None => {
                // info!("None")
            }
        }
        match encoder_2.direction() {
            Direction::Clockwise => {
                display.clear().ok();
                for c in ['c', 'l', 'o', 'c', 'k', '2'] {
                    display.print_char(c).ok();
                }
            }
            Direction::Anticlockwise => {
                display.clear().ok();
                for c in ['a', 'n', 't', 'i', '2'] {
                    display.print_char(c).ok();
                }
            }
            Direction::None => {
                // info!("None")
            }
        }
        unsafe {
            MODULE_STATE.borrow(cs).replace(Some(ModuleState {
                encoder_poll_duration,
                encoder_poll_alarm,
                encoder_1,
                encoder_2,
                display,
                osc_state,
                ..module_state
            }))
        }
    });
}

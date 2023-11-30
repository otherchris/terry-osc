use defmt::*;
const MAX_LENGTH: usize = 1000;
const MAX_AMP: u16 = 0xfff;

pub type Sample = [Option<u16>; MAX_LENGTH];

pub struct Osc {
    pub wave: Wave,
    pub sample_count: usize,
    pub max_amp: u16,
    pub sample: Sample,
    pub next: usize,
    pub last: u16,
    pub edit_flag: bool,
}

pub struct OscState {
    pub osc_1: Osc,
    pub osc_2: Osc,
    pub osc_3: Osc,
    pub osc_4: Osc,
    pub edit_index: usize,
}

pub enum Wave {
    Ramp,
    Triangle,
    Saw,
    None,
}

// pub fn reset_osc(osc: &mut Osc) -> () {
//     if !osc.edit_flag {
//         return;
//     }
//     info!("resetting");
//     info!("{}", osc.sample_count);
//     let sample = match osc.wave {
//         Wave::Ramp => ramp(osc.sample_count, osc.max_amp),
//         Wave::Triangle => triangle(osc.sample_count, osc.max_amp),
//         Wave::Saw => saw(osc.sample_count, osc.max_amp),
//         Wave::None => none(osc.sample_count),
//     };
//     osc.sample = sample;
//     osc.edit_flag = false;
//     osc.next = 0;
// }

pub fn advance_osc(osc: &mut Osc) -> u16 {
    let val = match osc.sample[osc.next] {
        Some(val) => val,
        None => osc.last,
    };
    let next = match osc.sample[osc.next] {
        Some(_) => osc.next + 1,
        None => 0,
    };
    osc.next = next;
    val
}

pub fn new_osc() -> Osc {
    Osc {
        wave: Wave::None,
        sample: blank_sample(),
        edit_flag: false,
        sample_count: 0,
        next: 0,
        last: 0,
        max_amp: 0,
    }
}

pub fn blank_sample() -> Sample {
    [None; MAX_LENGTH]
}

pub fn ramp(length: usize, max_amp: u16, sample: &mut Sample) -> () {
    let factor = max_amp / length as u16;
    for i in 0..length {
        sample[i] = Some(factor * i as u16);
    }
    sample[length - 1] = None;
}

fn saw(length: usize, max_amp: u16) -> [Option<u16>; MAX_LENGTH] {
    let mut sample: [Option<u16>; MAX_LENGTH] = [Some(0); MAX_LENGTH];
    let factor = max_amp / length as u16;
    for i in 0..length {
        sample[i] = Some(factor * (length - i) as u16);
    }
    sample[length - 1] = None;
    sample
}

fn triangle(length: usize, max_amp: u16) -> [Option<u16>; MAX_LENGTH] {
    let mut sample: [Option<u16>; MAX_LENGTH] = [Some(0); MAX_LENGTH];
    let factor = 2 * max_amp / length as u16;
    for i in 0..(length / 2) {
        sample[i] = Some(factor * i as u16);
    }
    for i in 0..(length / 2) {
        sample[length - i] = Some(factor * i as u16)
    }
    sample[length - 1] = None;
    sample
}

fn none(length: usize) -> [Option<u16>; MAX_LENGTH] {
    let mut sample: [Option<u16>; MAX_LENGTH] = [Some(0); MAX_LENGTH];
    sample[length - 1] = None;
    sample
}

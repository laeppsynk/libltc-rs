extern crate libc;

use libltc_rs::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    let sample_rate = 48000.0;
    let fps = 30.0;
    let standard = LTCTVStandard::LTCTV_525_60;
    let flags = *LtcBgFlags::default().set(LtcBgFlagsKind::LTC_USE_DATE);

    let config = LTCEncoderConfig {
        sample_rate,
        fps,
        standard,
        flags,
    };
    let mut encoder = match LTCEncoder::try_new(&config) {
        Ok(encoder) => encoder,
        Err(e) => {
            eprintln!("Error creating LTC encoder: {}", e);
            return;
        }
    };

    let timezone: Timezone = b"+00100".into();
    let initial_timecode = SMPTETimecode::new(timezone, 3, 1, 10, 0, 0, 0, 1);
    println!(
        "Initial timecode: {:}",
        timecode_to_string(&initial_timecode)
    );

    encoder.set_timecode(&initial_timecode);
    println!(
        "Current timecode: {:}",
        timecode_to_string(&encoder.get_timecode())
    );

    let duration = Duration::new(2, 0); // 10 seconds
    let start_time = Instant::now();

    let mut frame = LTCFrame::new();

    while start_time.elapsed() < duration {
        let timecode = encoder.get_timecode();
        LTCFrame::from_timecode_inplace(&mut frame, &timecode, standard, flags);
        encoder.set_frame(&frame);
        encoder.encode_frame();
        println!(
            "Current timecode gotten: {:}",
            timecode_to_string(&encoder.get_timecode())
        );
        std::thread::sleep(Duration::from_secs_f64(1.0 / fps));
        encoder.inc_timecode().unwrap();
    }

    match encoder.end_encode() {
        Ok(_) => println!("Encoding finished successfully."),
        Err(e) => eprintln!("Error ending encoding: {}", e),
    }
    encoder.reset();
}

fn timecode_to_string(timecode: &SMPTETimecode) -> String {
    format!(
        "{} {:02}:{:02}:{:02} {:02}:{:02}:{:02}:{:02}",
        "??",
        timecode.years(),
        timecode.months(),
        timecode.days(),
        timecode.hours(),
        timecode.minutes(),
        timecode.seconds(),
        timecode.frame()
    )
}

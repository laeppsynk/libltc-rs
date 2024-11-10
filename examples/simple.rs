extern crate libc;

use libltc_rs::{LTCEncoder, LTCFrame, LTCTVStandard, SMPTETimecode};
use std::time::{Duration, Instant};

fn main() {
    // Define the sample rate, fps, and TV standard
    let sample_rate = 48000.0;
    let fps = 30.0;
    let standard = LTCTVStandard::LTCTV_525_60;
    let flags = 0;

    // Create the LTC encoder
    let mut encoder = match LTCEncoder::try_new(sample_rate, fps, standard, flags) {
        Ok(encoder) => encoder,
        Err(e) => {
            eprintln!("Error creating LTC encoder: {}", e);
            return;
        }
    };

    fn slice_to_array(slice: &[u8]) -> Option<[u8; 6]> {
        if slice.len() == 6 {
            let mut arr = [0u8; 6];
            arr.copy_from_slice(slice);
            Some(arr)
        } else {
            None
        }
    }

    let timezone: [u8; 6] = b"+00100".to_owned();
    // Set an initial timecode
    let initial_timecode = SMPTETimecode::default();
    encoder.set_timecode(&initial_timecode);

    // Prepare to encode a sequence of frames
    let duration = Duration::new(10, 0); // 10 seconds
    let start_time = Instant::now();

    while start_time.elapsed() < duration {
        // Get the next frame's timecode
        let timecode = encoder.get_timecode();

        // Set the current frame
        let frame = LTCFrame::from_timecode(&timecode, standard, flags);
        encoder.set_frame(&frame);

        // Encode the frame
        encoder.encode_frame();

        // Print the current timecode
        println!(
            "Encoded timecode: {:02}:{:02}:{:02}:{:02}",
            timecode.hours, timecode.mins, timecode.secs, timecode.frame
        );

        // Simulate a small delay (time between frames)
        std::thread::sleep(Duration::from_secs_f64(1.0 / fps));

        encoder.inc_timecode();
    }

    // End encoding
    match encoder.end_encode() {
        Ok(_) => println!("Encoding finished successfully."),
        Err(e) => eprintln!("Error ending encoding: {}", e),
    }

    // Optionally, reset or free the encoder
    encoder.reset();
}

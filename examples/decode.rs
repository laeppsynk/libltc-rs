extern crate libc;

use libltc_rs::prelude::*;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process::exit;

const BUFFER_SIZE: usize = 1024;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    let mut apv = 1920;

    if args.len() > 1 {
        filename = &args[1];
        if args.len() > 2 {
            apv = args[2].parse().unwrap_or(1920);
        }
    } else {
        eprintln!(
            "Usage: {} <filename> [audio-frames-per-video-frame]",
            args[0]
        );
        exit(1);
    }

    // Open the file for reading
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error opening '{}'", filename);
            exit(1);
        }
    };

    eprintln!("* Reading from: {}", filename);

    let mut total = 0;
    let mut sound: Vec<SampleType> = vec![0; BUFFER_SIZE];

    // Create the LTC decoder
    let config = LTCDecoderConfig {
        apv,
        queue_size: 32,
    };
    let mut decoder = LTCDecoder::try_new(&config).unwrap();

    loop {
        let n = match file.read_exact(sound.as_mut_slice()) {
            Ok(_) => BUFFER_SIZE,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                break; // End of file reached
            }
            Err(_) => {
                eprintln!("Error reading from file.");
                exit(1);
            }
        };

        decoder.write(&sound[0..n], total);

        while let Some(frame) = decoder.read() {
            let flags = *LtcBgFlags::default().set(LtcBgFlagsKind::LTC_USE_DATE);
            // FIX: There's a double free here. ltc() should maybe be a copy?
            let stime = &frame.ltc().to_timecode(flags);

            // Print out the decoded timecode
            println!(
                "{:04}-{:02}-{:02} {} {:02}:{:02}:{:02}{:02} | {:8} {:8} {} {}",
                stime.years(),
                stime.months(),
                stime.days(),
                stime.timezone(),
                stime.hours(),
                stime.minutes(),
                stime.seconds(),
                if frame.ltc().dfbit() == 1 { '.' } else { ':' },
                stime.frame(),
                frame.off_start(),
                frame.off_end(),
                if frame.reverse() { "  R" } else { "" }
            );
        }

        total += n as i64;
    }

    eprintln!("Done: read {} samples from '{}'", total, filename);
}

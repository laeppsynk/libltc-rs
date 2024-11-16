/**
   @brief inspired by example code for libltc LTCEncoder
   @file ltcencode.c

   Original work by Robin Gareus <robin@gareus.org> and Jan <jan@geheimwerk.de>.
   This file is a rust example inspired by the original example code in C.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU Lesser Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with this library; if not, write to the Free Software
   Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA.
*/
extern crate libc;

use libltc_rs::prelude::*;
use std::io::Write;

use std::env;
use std::fs::File;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    let mut sample_rate = 48000.0;
    let mut fps = 25.0;
    let mut length = 1.0;

    if args.len() > 1 {
        filename = &args[1];
        if args.len() > 2 {
            sample_rate = args[2].parse().unwrap_or(48000.0);
        }
        if args.len() > 3 {
            fps = args[3].parse().unwrap_or(25.0);
        }
        if args.len() > 4 {
            length = args[4].parse().unwrap_or(2.0);
        }
    } else {
        eprintln!("encode - test/example application to encode LTC to a file\n");
        eprintln!(
            "Usage: {} <filename> [sample rate [frame rate [duration in s]]]\n",
            args[0]
        );
        eprintln!("default-values:");
        eprintln!(" sample rate: 48000.0 [SPS], frame rate: 25.0 [fps], duration: 2.0 [sec]\n");
        eprintln!("Report bugs to Robin Gareus <robin@gareus.org>\n");
        exit(1);
    }

    let file = match File::create(filename) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: cannot open file '{}' for writing.", filename);
            exit(1);
        }
    };

    // Initialize the timecode structure
    let timezone: &[u8; 6] = b"+0100\0";
    let timezone = timezone.into();
    println!("{}", timezone);

    let st = SMPTETimecode::new(timezone, 8, 12, 31, 23, 59, 59, 0);

    println!("{}", &st.timezone());
    let flags = *LtcBgFlags::default().set(LtcBgFlagsKind::LTC_USE_DATE);

    // Initialize the LTC Encoder
    let config = LTCEncoderConfig::default();
    let mut encoder = LTCEncoder::try_new(&config).unwrap();

    encoder.set_buffersize(sample_rate, fps).unwrap();
    encoder
        .reinit(
            sample_rate,
            fps,
            if fps == 25.0 {
                LTCTVStandard::LTCTV_625_50
            } else {
                LTCTVStandard::LTCTV_525_60
            },
            flags,
        )
        .unwrap();

    encoder.set_filter(25.0);
    encoder.set_volume(-18.0).unwrap();

    encoder.set_timecode(&st);

    println!("sample rate: {:.2}", sample_rate);
    println!("frames/sec: {:.2}", fps);
    println!("secs to write: {:.2}", length);
    println!("sample format: 8bit unsigned mono");

    let vframe_last = (length * fps) as i32;
    let mut total_samples = 0;
    let mut file = file;

    for _ in 0..vframe_last {
        encoder.encode_frame();

        let (buf, len) = encoder.get_buf_ref(true);

        // In the loop where you process frames
        if len > 0 {
            // Assuming buf is a slice of raw bytes or samples, you need to write this to the file
            match file.write_all(&buf[..len]) {
                Ok(_) => total_samples += len, // Increment the total samples written
                Err(e) => {
                    eprintln!("Error writing to file: {}", e);
                    exit(1);
                }
            }
        }
        encoder.inc_timecode().unwrap();
    }

    println!("Done: wrote {} samples to '{}'", total_samples, filename);
}

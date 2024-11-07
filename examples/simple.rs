use libltc_rs::*;

fn main() {
    // Set sample rate, FPS, standard, and flags
    let sample_rate = 48000.0;
    let fps = 29.97;
    let standard = LTCTVStandard::LTCTV_525_60;
    let flags = 0; // Adjust flags as necessary

    // Create the LTC encoder instance
    let mut encoder = LTCEncoder::try_new(sample_rate, fps, standard, flags).unwrap();

    // Set the filter with a specified rise time
    let rise_time = 0.5; // Example rise time
    encoder.set_filter(rise_time);
    println!("Successfully set filter with rise time: {}", rise_time);

    // Encoder will be automatically dropped when it goes out of scope
}

extern crate hound;
use std::env;

fn main() {
    println!("FFT Compute - Jeff Cowgill");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let filename = &args[1];
        println!("filename: {:?}", filename);
        let mut _reader = hound::WavReader::open(filename).unwrap();
    } else {
        println!("fft_compute takes 1 argument, wav filename");
    }
}

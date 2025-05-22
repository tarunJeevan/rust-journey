extern crate flate2;

use flate2::Compression;
use flate2::write::GzEncoder;
// use flate2::write::ZlibEncoder;

use std::env::args; // NOTE: Replace with Clap?
use std::fs::File;
use std::io::BufReader;
use std::io::copy;
use std::time::Instant;

// TODO: Allow decoding with a -d flag
// TODO: Allow user to choose between different encoders and decoders

fn main() {
    // NOTE: Replace with Clap?
    if args().len() != 3 {
        eprintln!("Usage: file_compressor <input_file> <output_file>");
    }

    // Create input and output files
    let mut input_file = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output_file = File::create(args().nth(2).unwrap()).unwrap();
    // Create encoder
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    // Start timer
    let start = Instant::now();

    // Encoding process
    copy(&mut input_file, &mut encoder).unwrap();
    let output_file = encoder.finish().unwrap();

    // Print file data
    println!(
        "Source file size: {:?} bytes",
        input_file.get_ref().metadata().unwrap().len()
    );
    println!(
        "Compressed file size: {:?} bytes",
        output_file.metadata().unwrap().len()
    );
    println!("Time elapsed: {:?}", start.elapsed());

    // TODO: Decoding?
}

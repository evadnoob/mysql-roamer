use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write, stdout};

pub fn main() {
let mut output : BufWriter<Box<Write>> = 
    BufWriter::new(if let Some(filename) = env::args().nth(1) { 
        Box::new(File::create(filename).unwrap())
    } else {
        Box::new(stdout())
    });

    output.write(b"somethings ....");
    output.flush();
}

use std::fs::File;
use std::io::{BufRead, BufReader};

use flate2::read::GzDecoder;

pub struct NCount {
    data: [i64; 5],
}
impl NCount {
    pub fn new() -> Self {
        NCount {
            data: [0, 0, 0, 0, 0],
        }
    }
    pub fn add(&mut self, c: &u8) {
        match c {
            b'A' | b'a' => self.data[0] += 1,
            b'C' | b'c' => self.data[1] += 1,
            b'G' | b'g' => self.data[2] += 1,
            b'T' | b't' => self.data[3] += 1,
            _ => self.data[4] += 1,
        }
    }
    pub fn print(&mut self) {
        let [a, c, g, t, n] = self.data;
        println!("A: {}; C: {}; G: {}; T: {}; N: {}", a, c, g, t, n);
    }
}

pub fn open_bufreader(path: &str) -> Result<Box<dyn BufRead>, std::io::Error> {
    let file = File::open(path)?;
    if path.ends_with(".gz") {
        let decoder = GzDecoder::new(file);
        Ok(Box::new(BufReader::new(decoder)))
    } else {
        Ok(Box::new(BufReader::new(file)))
    }
}

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    // Codes für die kleinen Zeichen: - 96
    // Codes für die großen Zeichen: - 38
    
    let file = File::open("res/day3.txt").unwrap();
    let reader = BufReader::new(file);
}

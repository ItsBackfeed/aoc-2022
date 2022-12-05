use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let filename = "res/day1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    println!("Solving Day1");

    let mut highest: u32 = 0;
    let mut temp_high: u32 = 0;
    let mut vec:Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.as_str() == "" {
            temp_high = 0
        } else {
            temp_high += line.parse::<u32>().expect("Bled");
            vec.push(temp_high);
        }

        if temp_high > highest {
            highest = temp_high;
        }
    }

    println!("Solution: {}", highest);
    vec.sort();
    vec.reverse();
    println!("Second solution: {}", vec[0] + vec[1] + vec[2]);
}

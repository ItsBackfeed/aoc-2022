use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    // Codes für die kleinen Zeichen: - 96
    // Codes für die großen Zeichen: - 38
    
    let file = File::open("res/day3.txt").unwrap();
    let reader = BufReader::new(file);

    let mut score: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let left_part = &line[0..line.len()/2];
        let right_part= &line[line.len()/2..line.len()];

        for i in left_part.chars() {
            let mut found_chars:Vec<char> = Vec::new();
            for j in right_part.chars() {
                if i == j {
                    if !found_chars.contains(&j) {
                        if i.is_ascii_lowercase() {score += i as u32 - 96}
                        else  {score += i as u32 - 38}
                        found_chars.push(j);
                    }
                }
            }
        }
    }

    println!("Solution 1: {}", score);
}

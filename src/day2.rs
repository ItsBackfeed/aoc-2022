use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let file = File::open("res/day2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut totalscore: u32 = 0;
    let mut totalscore2: u32 = 0;
    for line in reader.lines() {
        let line_chars: Vec<char> = line.unwrap().chars().collect();

        totalscore += compare(line_chars[0], line_chars[2]);
        totalscore2 += compare2(line_chars[0], line_chars[2]);
    }

    println!("Solution 1: {}", totalscore);
    println!("Solution 2: {}", totalscore2);
}

fn compare2 (elf: char, sign: char) -> u32 {
    let mut return_value = 0;

    match sign {
        'Z' => {
            return_value += 6;
            match elf {
                'A' => return_value += 2,
                'B' => return_value += 3,
                _ => return_value += 1,
            }
        },
        'Y' => {
            return_value += 3;
            match elf{
                'A' => return_value += 1,
                'B' => return_value += 2,
                _ => return_value += 3,
            }
        }
        _ => {
            return_value += 0;
            match elf {
                'A' => return_value += 3,
                'B' => return_value += 1,
                _ => return_value += 2,
            }
        },
    }


    return return_value;
}

fn compare (elf: char, you: char) -> u32 {
    let mut return_value = 0;

    match you {
        'X' => {
            return_value += 1;
            match elf {
                'A' => return_value += 3,
                'C' => return_value += 6,
                _ => return_value += 0,
            }
        }
        'Y' => {
            return_value += 2;
            match elf {
                'A' => return_value += 6,
                'B' => return_value += 3,
                _ => return_value += 0,
            }
        }
        'Z' => {
            return_value += 3;
            match elf {
                'C' => return_value += 3,
                'B' => return_value += 6,
                _ => return_value += 0,
            }
        }
        _ => return_value = 0,
    };

    return return_value;
}

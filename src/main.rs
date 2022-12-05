use std::io;

pub mod day1;

fn main() {
    println!("Please choose the day:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error getting input!");

    let day: u32 = input.trim().parse().expect("Only numbers are accepted!");
    
    match day {
        1 => day1::solve(),
        _=> println!("Seas"),
    };
}

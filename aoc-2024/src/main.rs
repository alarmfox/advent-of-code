use core::panic;
use std::{env, fs};

mod puzzles;
fn main() {
    let args = env::args().collect::<Vec<String>>();

    match args.len() {
        2 => {
            let arg = args[1].as_str();
            let (day, part) = arg.split_once("-").unwrap();
            let input = fs::read_to_string(format!("input/{}.in", day)).unwrap();
            let res = match arg {
                "01-a" => puzzles::day01::part_a(input.as_str()),
                "01-b" => puzzles::day01::part_b(input.as_str()),
                "02-a" => puzzles::day02::part_a(input.as_str()),
                "02-b" => puzzles::day02::part_b(input.as_str()),
                _ => panic!("Unknown day"),
            };
            println!("{}", res);
        }
        _ => {}
    }
}

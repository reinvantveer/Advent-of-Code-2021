mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

use clap::{App, Arg};

pub fn main() {
    let matches = App::new("Advent of Code 2021")
        .version("0.1.0")
        .author("Rein van 't Veer <rein@vantveer.me>")
        .about("My solutions to the Advent of Code 2021 - see https://adventofcode.com/2021")
        .arg(Arg::with_name("day")
            .long("day")
            .short("d")
            .required(true)
            .takes_value(true)
            .help("Which day of the advent to run"))
        .get_matches();

    let day = matches.value_of("day").unwrap();
    println!("Running solutions for day {}", &day);
    match day {
        "1" => { day_1::run() },
        "2" => { day_2::run() },
        "3" => { day_3::run() },
        "4" => { day_4::run() },
        "5" => { day_5::run() },
        "6" => { day_6::run() },
        "7" => { day_7::run() },
        "8" => { day_8::run() },
        _ => { todo!("This day isn't implemented (yet)") },
    };
}

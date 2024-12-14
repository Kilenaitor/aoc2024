mod days;

use days::{day::Day, day4::Day4, day5::Day5, day6::Day6};

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg()]
    day: u8,

    #[arg(short, long, value_enum, default_value_t = Part::Both)]
    part: Part,

    #[arg(short = 't', long, default_value_t = false)]
    is_test: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Part {
    P1,
    P2,
    Both,
}

fn main() {
    let cli = Cli::parse();

    let day: Box<dyn Day> = match &cli.day {
        4 => Box::new(Day4 {}),
        5 => Box::new(Day5 {}),
        6 => Box::new(Day6 {}),
        _ => panic!("Not implemented"),
    };

    day.run(&cli.part, cli.is_test);
}

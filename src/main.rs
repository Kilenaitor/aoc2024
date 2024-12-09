mod days;

use days::{day::Day, day4::Day4};

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    day: AOCDay,
}

#[derive(Subcommand)]
enum AOCDay {
    D4 {
        #[arg(short, long, value_enum, default_value_t = Part::Both)]
        part: Part,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Part {
    P1,
    P2,
    Both,
}

fn main() {
    let cli = Cli::parse();

    let (day, part) = match &cli.day {
        AOCDay::D4 { part } => (Day4 {}, part),
    };

    day.run(part);
}

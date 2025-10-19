use crate::log_level::Levels;
use crate::time::Time;
use clap::Parser;
use colored::ColoredString;
use std::io;
use std::io::Read;
use crate::color::color_message;

mod color;
mod config;
mod log_level;
mod time;

#[derive(Parser)]
#[command(name = "logm", version, about = "Logm", arg_required_else_help = false)]
pub struct Args {
    #[arg(help = "")]
    level: Option<Levels>,

    #[arg(help = "")]
    time: Option<Time>,
}

fn main() {
    let args = Args::parse();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    if atty::isnt(atty::Stream::Stdout) {
        for s in buffer.trim().split("\n") {
            println!("{} {} {}", args.time.unwrap_or_default(), args.level.unwrap_or_default(), s);

            return;
        }
    }

    let time = ColoredString::from(args.time.unwrap_or_default());
    let level = ColoredString::from(args.level.unwrap_or_default());

    for s in buffer.trim().split("\n") {
        println!("{} {} {}", time, level, color_message(s));
    }
}

mod converter;

use serde::{Deserialize, Serialize};
use clap::Parser;
use converter::FileTypes;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    fname: String,
    #[arg(long)]
    from_type: FileTypes,
    #[arg(long)]
    to_type: FileTypes
}

#[derive(Debug, Serialize, Deserialize)]
struct Ship {
    title: String,
    factory: String,
    project: String
}

fn main() {
    let args: Args = Args::parse();

    if args.from_type == args.to_type {
        println!("Formats are the same");
        return;
    }

    let ship: Ship = converter::read_file(&args);
    converter::write_object(&ship, &args);

    println!("{:?}", ship);
}

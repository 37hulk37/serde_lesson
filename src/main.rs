use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use serde::{Deserialize, Serialize};
use clap::Parser;
use strum::{Display, EnumString};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    fname: String,
    #[arg(long)]
    from_type: FileTypes,
    #[arg(long)]
    to_type: FileTypes
}

#[derive(EnumString, Display, Serialize, Deserialize, PartialEq, Debug, Clone)]
enum FileTypes {
    JSON(String),
    YAML(String),
    RON(String)
}

#[derive(Debug, Serialize, Deserialize)]
struct Ship {
    title: String,
    factory: String,
    project: String
}

fn build_file_path(fname: &str, ftype: &str) -> String {
    let mut path = String::new();
    path.push_str("./resources/");

    path + fname + "." + ftype.to_lowercase().as_str()
}

fn read_file(args: &Args) -> Ship {
    let path = build_file_path(args.fname.as_str(), args.from_type.to_string().as_str());
    let file = File::open(path.as_str()).unwrap();
    let reader = BufReader::new(file);

    match args.from_type {
        FileTypes::JSON(_) => parse_json(reader),
        FileTypes::YAML(_) => parse_yaml(reader),
        FileTypes::RON(_) => panic!()
    }
}

fn parse_json(reader: BufReader<File>) -> Ship {
    serde_json::from_reader(reader).unwrap()
}

fn parse_yaml(reader: BufReader<File>) -> Ship {
    serde_yaml::from_reader(reader).unwrap()
}

fn write_ship(ship: &Ship, args: &Args) {
    let path = build_file_path(args.fname.as_str(), args.to_type.to_string().as_str());
    let file = File::create(path).unwrap();
    let writer = BufWriter::new(file);

    match args.to_type {
        FileTypes::JSON(_) => write_json(&ship, writer),
        FileTypes::YAML(_) => write_yaml(&ship, writer),
        FileTypes::RON(_) => panic!()
    }
}

fn write_yaml(ship: &Ship, mut writer: BufWriter<File>) {
    match writer.write_all(serde_yaml::to_string(ship).unwrap().as_bytes()) {
        Ok(()) => writer.flush().unwrap(),
        Err(err) => panic!("{}", err)
    }
}

fn write_json(ship: &Ship, mut writer: BufWriter<File>) {
    match writer.write_all(serde_json::to_string(ship).unwrap().as_bytes()) {
        Ok(()) => writer.flush().unwrap(),
        Err(err) => panic!("{}", err)
    }
}

fn main() {
    let args: Args = Args::parse();

    if args.from_type == args.to_type {
        println!("Formats are same");
        return;
    }

    let ship = read_file(&args);
    write_ship(&ship, &args);

    println!("{:?}", ship);
}

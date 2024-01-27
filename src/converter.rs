use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use crate::Args;
use crate::converter::FileTypes::{JSON, TOML, YAML};

#[derive(EnumString, Display, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum FileTypes {
    JSON(String),
    YAML(String),
    TOML(String)
}

pub fn read_file<T: for<'a> Deserialize<'a>>(args: &Args) -> T {
    let path = build_file_path(args.fname.as_str(), args.from_type.to_string().as_str());
    let file = File::open(path.as_str()).unwrap();
    let reader = BufReader::new(file);

    match args.from_type {
        JSON(_) => parse_json(reader),
        YAML(_) => parse_yaml(reader),
        TOML(_) => panic!()
    }
}

pub fn write_object<T: Serialize>(object: &T, args: &Args) {
    let path = build_file_path(args.fname.as_str(), args.to_type.to_string().as_str());
    let file = File::create(path).unwrap();
    let writer = BufWriter::new(file);

    match args.to_type {
        JSON(_) => write_json(&object, writer),
        YAML(_) => write_yaml(&object, writer),
        TOML(_) => panic!("TOML is unsupported")
    }
}

fn build_file_path(fname: &str, ftype: &str) -> String {
    let mut path = String::new();
    path.push_str("./resources/");

    path + fname + "." + ftype.to_lowercase().as_str()
}

fn parse_json<T: for<'a> Deserialize<'a>>(reader: BufReader<File>) -> T {
    serde_json::from_reader(reader).unwrap()
}

fn parse_yaml<T: for<'a> Deserialize<'a>>(reader: BufReader<File>) -> T {
    serde_yaml::from_reader(reader).unwrap()
}

fn write_yaml<T: Serialize>(object: &T, mut writer: BufWriter<File>) {
    match writer.write_all(serde_yaml::to_string(object).unwrap().as_bytes()) {
        Ok(()) => writer.flush().unwrap(),
        Err(err) => panic!("{}", err)
    }
}

fn write_json<T: Serialize>(object: &T, mut writer: BufWriter<File>) {
    match writer.write_all(serde_json::to_string(object).unwrap().as_bytes()) {
        Ok(()) => writer.flush().unwrap(),
        Err(err) => panic!("{}", err)
    }
}
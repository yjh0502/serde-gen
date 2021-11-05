extern crate clap;
extern crate serde;
extern crate serde_json;

extern crate serde_gen;

use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use clap::{App, Arg};
use log::*;
use serde_gen::*;

fn main() -> Result<()> {
    env_logger::init();

    let matches = App::new("serde_gen")
        .version("0.1")
        .author("Jihyun Yu <yjh0502@gmail.com")
        .arg(
            Arg::with_name("out")
                .long("out")
                .takes_value(true)
                .help("output rust filename, standard output if not exists"),
        )
        .arg(
            Arg::with_name("ndjson")
                .long("ndjson")
                .takes_value(false)
                .help("accepts ndjson format"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .multiple(true),
        )
        .get_matches();

    let mut ty: Ty = Ty::Unit;

    if !matches.is_present("INPUT") {
        ty = ty + serde_json::from_reader(std::io::stdin())?;
    }

    for filename in matches.values_of("INPUT").unwrap() {
        let mut f = File::open(filename)?;

        if matches.is_present("ndjson") {
            let mut reader = BufReader::new(f);
            let mut s = String::new();
            loop {
                let n = reader.read_line(&mut s)?;
                if n == 0 {
                    break;
                }
                ty = ty + serde_json::from_str(&s)?;
                s.clear();
            }
        } else {
            ty = ty + serde_json::from_reader(&mut f)?;
        }
    }
    debug!("ty={:?}", ty);

    let mut builder = TyBuilder::new();
    let out = builder.build("Root", ty);
    debug!("out={}", out);

    match matches.value_of("out") {
        Some(filename) => write!(&mut File::create(filename)?, "{}\n", out)?,
        None => write!(&mut std::io::stdout(), "{}\n", out)?,
    };

    Ok(())
}

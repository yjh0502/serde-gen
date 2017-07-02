extern crate serde;
extern crate serde_json;
extern crate clap;

extern crate serde_gen;

use std::io::Write;
use std::fs::File;

use serde_gen::*;
use clap::{App, Arg};

fn run() -> Result<()> {
    let matches = App::new("serde_gen")
        .version("0.1")
        .author("Jihyun Yu <yjh0502@gmail.com")
        .arg(Arg::with_name("out").long("out").takes_value(true).help(
            "output rust filename, standard output if not exists",
        ))
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
        ty = ty + serde_json::from_reader(&mut f)?;
    }

    let mut builder = TyBuilder::new();
    let out = builder.build(ty);
    match matches.value_of("out") {
        Some(filename) => write!(&mut File::create(filename)?, "{}\n", out)?,
        None => write!(&mut std::io::stdout(), "{}\n", out)?,
    };
    Ok(())
}

fn main() {
    run().expect("failed to run");
}

extern crate serde;
extern crate serde_json;
extern crate clap;
#[macro_use]
extern crate error_chain;

extern crate serde_gen;

use std::fs::File;
use std::io::prelude::*;

use serde_gen::*;
use clap::{App, Arg};

error_chain!{
    foreign_links {
        Io(std::io::Error);
        Json(serde_json::Error);
    }
}

fn run() -> Result<()> {
    let matches = App::new("serde_gen")
        .version("0.1")
        .author("Jihyun Yu <yjh0502@gmail.com")
        .arg(Arg::with_name("in")
                 .long("in")
                 .required(true)
                 .takes_value(true))
        .arg(Arg::with_name("out")
                 .long("out")
                 .required(true)
                 .takes_value(true))
        .get_matches();

    let in_filename = matches.value_of("in").ok_or("no input file")?;
    let out_filename = matches.value_of("out").ok_or("no input file")?;

    let mut file = File::open(in_filename)?;
    let out_file = File::create(out_filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let v: Ty = serde_json::from_str(&contents)?;

    let mut builder = TyBuilder::new();
    let code = builder.build(v);

    write!(&out_file, "{}\n", code)?;
    Ok(())
}

fn main() {
    run().expect("failed to run");
}

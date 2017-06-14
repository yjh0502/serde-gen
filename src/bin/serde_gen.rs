extern crate serde;
extern crate serde_json;
extern crate clap;
#[macro_use]
extern crate error_chain;

extern crate serde_gen;

use std::fs::File;

use serde_gen::*;
use clap::{App, Arg};

error_chain!{
    foreign_links {
        Io(std::io::Error);
        Json(serde_json::Error);
    }
}

fn run_translate<R, W>(r: &mut R, w: &mut W) -> Result<()>
    where R: std::io::Read,
          W: std::io::Write
{
    let v: Ty = serde_json::from_reader(r)?;

    let mut builder = TyBuilder::new();
    write!(w, "{}\n", builder.build(v))?;
    Ok(())
}

fn run() -> Result<()> {
    let matches = App::new("serde_gen")
        .version("0.1")
        .author("Jihyun Yu <yjh0502@gmail.com")
        .arg(Arg::with_name("in")
                 .long("in")
                 .takes_value(true)
                 .help("input JSON filename, standard input if not exists"))
        .arg(Arg::with_name("out")
                 .long("out")
                 .takes_value(true)
                 .help("output rust filename, standard output if not exists"))
        .get_matches();

    match (matches.value_of("in"), matches.value_of("out")) {
        (Some(in_filename), Some(out_filename)) => {
            run_translate(&mut File::open(in_filename)?,
                          &mut File::create(out_filename)?)
        }
        (Some(in_filename), None) => {
            run_translate(&mut File::open(in_filename)?, &mut std::io::stdout())
        }
        (None, Some(out_filename)) => {
            run_translate(&mut std::io::stdin(), &mut File::create(out_filename)?)
        }
        (None, None) => run_translate(&mut std::io::stdin(), &mut std::io::stdout()),
    }
}

fn main() {
    run().expect("failed to run");
}

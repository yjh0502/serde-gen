extern crate serde;
extern crate serde_json;

extern crate serde_gen;

use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use log::*;
use serde_gen::*;

use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Reach new heights.
struct Args {
    /// how high to go
    #[argh(option)]
    out: Option<String>,

    #[argh(switch, description = "input is ndjson")]
    ndjson: bool,

    #[argh(positional)]
    inputs: Vec<String>,
}

fn main() -> Result<()> {
    env_logger::init();

    let args: Args = argh::from_env();
    info!("args={:?}", args);

    let mut ty: Ty = Ty::Unit;

    if args.inputs.is_empty() {
        ty = ty + serde_json::from_reader(std::io::stdin())?;
    }

    for filename in args.inputs {
        let mut f = File::open(filename)?;

        if args.ndjson {
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

    match args.out {
        Some(filename) => write!(&mut File::create(filename)?, "{}\n", out)?,
        None => write!(&mut std::io::stdout(), "{}\n", out)?,
    };

    Ok(())
}

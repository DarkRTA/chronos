use clap::Parser;
use livesplit_core::run::parser::composite;
use livesplit_core::{Run, Segment};
use std::fs;
use std::path::Path;

#[derive(Parser)]
pub struct Args {
    pub splits_file: String,
}

pub fn parse_args() -> Args {
    return Args::parse();
}

pub fn parse_splits_file(args: Args) -> Run {
    let args = &args.splits_file;
    let path = Path::new(&args);

    match fs::read(path) {
        Ok(file) => {
            let parsed =
                composite::parse(&file, None).expect("Not a valid splits file");
            parsed.run
        }
        Err(_) => default_run(),
    }
}

// creates a default run.
// run::Editor does not accept a run without any segments
fn default_run() -> Run {
    let mut run = Run::default();
    run.push_segment(Segment::new("Test"));
    run
}

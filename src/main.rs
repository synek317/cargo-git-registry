#[macro_use]
extern crate structopt;
extern crate url;
#[macro_use]
extern crate failure;
extern crate git2;
extern crate directories;
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate relative_path;
extern crate regex;

mod cmd_args;
mod commands;
mod repo;
mod utils;

use structopt::StructOpt;
use std::process::exit;
use failure::Error;

fn main() {
    if let Err(e) = run() {
        let mut causes = e.causes();

        if let Some(root_cause) = causes.next() {
            eprintln!("ERROR: {}", root_cause);

            for cause in causes {
                eprintln!("  caused by: {}", cause);
            }
        }
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    use cmd_args::CmdArgs::*;

    let cmd = cmd_args::CmdArgs::from_args();

    match cmd {
        Clone(args)    => commands::clone(args),
        Forget(args)   => commands::forget(args),
        Init(args)     => commands::init(args),
        List           => commands::list(),
        Publish(args)  => commands::publish(args),
        Remember(args) => commands::remember(args),
        Remove(args)   => commands::remove(args),
        Update(args)   => commands::update(args),
        Use(args)      => commands::use_registry(args),
    }
}

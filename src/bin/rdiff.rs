/// rdiff command-line tool

// Copyright 2018 Martin Pool.

#[macro_use]
extern crate clap;
extern crate rdiff;

use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Result, stdin, stdout};

use clap::{AppSettings, Arg, ArgMatches, SubCommand};

use rdiff::mksum::{SignatureOptions, generate_signature};

pub fn main() {
    let app = app_from_crate!()
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("signature")
            .about("Generate a signature file from a basis")
            .arg(Arg::with_name("basis")
                .required(true)
                .help("Basis file to read, or - for stdin"))
            .arg(Arg::with_name("signature")
                .required(true)
                .help("Signature file to write, or - for stdout")));


    let r = match app.get_matches().subcommand() {
        ("signature", Some(subm)) => signature_cmd(&subm),
        _ => unimplemented!(), // shouldn't happen
    };
    if let Err(e) = r {
        eprintln!("rdiff error: {:?}", e);
        std::process::exit(1);
    };
}

fn signature_cmd(subm: &ArgMatches) -> Result<()> {
    let mut basis = open_input(subm.value_of_os("basis").unwrap())?;
    let mut sig = open_output(subm.value_of_os("signature").unwrap())?;
    let options = SignatureOptions::default();
    generate_signature(&mut basis, &options, &mut sig)
}

/// Open a file from a file name for input, treating `-` as stdin.
fn open_input(n: &OsStr) -> Result<Box<Read>> {
    match n.to_str() {
        Some("-") => Ok(Box::new(stdin())),
        _ => match File::open(n) {
            Ok(f) => Ok(Box::new(f)),
            Err(e) => {
                eprintln!("rdiff: can't open input {:?}: {}", n, e);
                Err(e)
            }
        }
    }
}

/// Open a file from a file name for output, treating `-` as stdout.
fn open_output(n: &OsStr) -> Result<Box<Write>> {
    match n.to_str() {
        Some("-") => Ok(Box::new(stdout())),
        _ => match File::create(n) {
            Ok(f) => Ok(Box::new(f)),
            Err(e) => {
                eprintln!("rdiff: can't create output {:?}: {}", n, e);
                Err(e)
            }
        }
    }
}

#![allow(dead_code)]

mod client;
mod error;

use actix_rt::{Arbiter, System};
use exitfailure::ExitFailure;
use futures::future::{self, Future};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CmdLineOpts {
    artist: String,
    song: String,
}

fn main() -> Result<(), ExitFailure> {
    let cmo = CmdLineOpts::from_args();

    let sys = System::new("az-fetcher");

    Arbiter::spawn(future::lazy(move || {
        client::Client::default()
            .fetch_lyric(&cmo.artist, &cmo.song)
            .map(|html| println!("{}", html))
            .map_err(|e| println!("{:?}", e))
            .then(|_| {
                System::current().stop();
                Ok(())
            })
    }));

    Ok(sys.run()?)
}

#[test]
fn cli_basic_usage() {
    use assert_cmd::cargo::CommandCargoExt;
    use std::process::Command;

    let stdout = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("slipknot")
        .arg("unsainted")
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap()
        .stdout;
    let _stdout = String::from_utf8(stdout).unwrap();

    unimplemented!("TODO: validade stdout");
}

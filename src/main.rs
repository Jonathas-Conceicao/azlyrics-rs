mod client;
mod data;
mod error;

use actix_rt::System;
use structopt::StructOpt;

/// A CLI crawler to fetch music lyrics from www.azlyrics.com
#[derive(Debug, StructOpt)]
struct CmdLineOpts {
    /// A already formated artist name, e.g., slipknot
    artist: String,
    /// A already formated song from the artist, e.g., unsainted
    song: String,
}

fn main() {
    System::new("azlyrics").block_on(async {
        let cmo = CmdLineOpts::from_args();

        let html = client::Client::default()
            .fetch_lyric(&cmo.artist, &cmo.song)
            .await
            .expect("Failed to fetch html");
        let data = data::Data::from_raw_html(html).expect("Unable to parse data");

        println!("{}", data.lyrics);

        System::current().stop()
    })
}

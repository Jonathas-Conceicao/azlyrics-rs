mod client;
mod data;

use actix_rt::{Arbiter, System};
use futures::future::{self, Future};
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
    let cmo = CmdLineOpts::from_args();

    let sys = System::new("azlyrics");

    Arbiter::spawn(future::lazy(move || {
        client::Client::default()
            .fetch_lyric(&cmo.artist, &cmo.song)
            .and_then(|html| Ok(data::Data::from_raw_html(html)))
            .map(|data| println!("{}", data.lyrics))
            .map_err(|e| println!("{:?}", e))
            .then(|_| {
                System::current().stop();
                Ok(())
            })
    }));

    sys.run().unwrap()
}

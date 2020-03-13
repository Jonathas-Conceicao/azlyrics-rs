mod client;
mod data;

use structopt::StructOpt;

/// A CLI crawler to fetch music lyrics from www.azlyrics.com
#[derive(Debug, StructOpt)]
struct CmdLineOpts {
    /// A already formated artist name, e.g., slipknot
    artist: String,
    /// A already formated song from the artist, e.g., unsainted
    song: String,
}

#[actix_rt::main]
async fn main() {
    let cmo = CmdLineOpts::from_args();
    let html = client::Client::default()
        .fetch_lyric(&cmo.artist, &cmo.song)
        .await;
    let data = data::Data::from_raw_html(html);
    println!("{}", data.lyrics);
}

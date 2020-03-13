use awc::http;
use futures::future::Future;

const USER_AGENT: &str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.8; rv:21.0) Gecko/20100101 Firefox/21.0";

pub(super) struct Client {
    server: String,
    client: awc::Client,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            server: String::from("https://www.azlyrics.com/lyrics"),
            client: awc::Client::build()
                .connector(
                    awc::Connector::new()
                        .ssl(
                            openssl::ssl::SslConnector::builder(openssl::ssl::SslMethod::tls())
                                .expect("Unable to build SSL connector!")
                                .build(),
                        )
                        .timeout(std::time::Duration::from_secs(3))
                        .finish(),
                )
                .header(http::header::ACCEPT, http::header::Accept::text())
                .header(http::header::USER_AGENT, USER_AGENT)
                .finish(),
        }
    }
}

impl Client {
    pub(super) fn fetch_lyric(
        &self,
        artist: &str,
        song: &str,
    ) -> impl Future<Item = String, Error = ()> {
        self.client
            .get(format!("{}/{}/{}.html", self.server, artist, song))
            .send()
            .map(|mut res| res.body())
            .map_err(|e| panic!("Client request failed: {:?}", e))
            .and_then(|body| body.map_err(|e| panic!("Failed to read body: {:?}", e)))
            .and_then(|bytes| Ok(String::from(std::str::from_utf8(&bytes).unwrap())))
    }
}

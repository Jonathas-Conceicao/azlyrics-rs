use crate::error::Result;
use awc::http;

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
    pub async fn fetch_lyric(&self, artist: &str, song: &str) -> Result<String> {
        let bytes = self
            .client
            .get(format!("{}/{}/{}.html", self.server, artist, song))
            .send()
            .await?
            .body()
            .await?;

        Ok(String::from(std::str::from_utf8(&bytes)?))
    }
}

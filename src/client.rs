use awc::http::header;

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
                .header(header::ACCEPT, header::Accept::text())
                .header(header::USER_AGENT, USER_AGENT)
                .finish(),
        }
    }
}

impl Client {
    pub(super) async fn fetch_lyric(&self, artist: &str, song: &str) -> String {
        let body = self
            .client
            .get(format!("{}/{}/{}.html", self.server, artist, song))
            .send()
            .await
            .expect("Client request failed")
            .body()
            .await
            .expect("Failed to read body");
        String::from_utf8(body.to_vec()).unwrap()
    }
}

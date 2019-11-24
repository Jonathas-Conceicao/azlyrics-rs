use crate::error::{Error, Result};
use select::{document::Document, predicate::Class};

pub(super) struct Data {
    pub(super) lyrics: String,
}

impl Data {
    pub(super) fn from_raw_html(html: String) -> Result<Self> {
        let doc = Document::from_read(html.as_bytes())?;
        let lyrics = lyric_in_html(doc)
            .ok_or(Error::DataError)?
            .trim()
            .to_string();

        Ok(Data { lyrics })
    }
}

fn lyric_in_html(doc: Document) -> Option<String> {
    Some(
        doc.find(Class("lyricsh"))
            .next()?
            .parent()?
            .children()
            .filter(|node| node.name().map(|n| n == "div").unwrap_or(false))
            .find(|node| node.attrs().next().is_none())?
            .text(),
    )
}

use regex::Regex;
use std::{collections::HashSet, error::Error};

const PAGE_URL: &str = "http://taharut.org/imo/";

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Document {
    pub name: String,
    pub author: String,
    pub url: String,
}

impl Document {
    pub fn build(name: String, author: String, url: String) -> Self {
        Document { name, author, url }
    }
}

#[derive(Clone)]
pub struct Documents(HashSet<Document>);

impl Documents {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Documents(HashSet::new())
    }

    pub fn get(&self) -> &HashSet<Document> {
        &self.0
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let res = ureq::get(PAGE_URL).call()?.into_string()?;

        let re = Regex::new(r#"<a href = "([^"]+)">([^<]+)</a>([^<]+)<br>"#)?;

        self.0 = re
            .captures_iter(&res)
            .map(|c| {
                let c = c.extract::<3>().1;
                Document::build(
                    c[1].trim().to_string(),
                    c[2].trim().to_string(),
                    PAGE_URL.to_string() + c[0],
                )
            })
            .collect();

        Ok(())
    }
}

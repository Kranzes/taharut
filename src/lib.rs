use regex::Regex;
use serde::Serialize;
use std::{collections::HashSet, error::Error};

const PAGE_URL: &str = "http://taharut.org/imo/";

#[derive(Hash, Eq, PartialEq, Clone, Serialize)]
pub struct Document {
    pub name: String,
    pub url: String,
}

impl Document {
    pub fn build(name: String, url: String) -> Self {
        Document { name, url }
    }
}

#[derive(Clone, Serialize)]
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

        let re = Regex::new(r#"([^"]*\.pdf)"#)?;

        self.0 = re
            .find_iter(&res)
            .map(|d| {
                Document::build(
                    d.as_str()
                        .strip_suffix(".pdf")
                        .unwrap()
                        .replace('_', " ")
                        .to_lowercase(),
                    PAGE_URL.to_string() + d.as_str(),
                )
            })
            .collect();

        Ok(())
    }
}

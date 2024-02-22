use regex::Regex;
use std::{collections::HashSet, error::Error};

const PAGE_URL: &str = "http://taharut.org/imo/";

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Exercise {
    pub name: String,
    pub author: String,
    pub url: String,
}

impl Exercise {
    pub fn build(name: String, author: String, url: String) -> Self {
        Exercise { name, author, url }
    }
}

#[derive(Clone)]
pub struct Exercises(HashSet<Exercise>);

impl Exercises {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Exercises(HashSet::new())
    }

    pub fn get(&self) -> &HashSet<Exercise> {
        &self.0
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let res = ureq::get(PAGE_URL).call()?.into_string()?;

        let re = Regex::new(r#"<a href = "([^"]+)">([^<]+)</a>([^<]+)<br>"#)?;

        self.0 = re
            .captures_iter(&res)
            .map(|c| {
                let c = c.extract::<3>().1;
                Exercise::build(
                    c[1].trim().to_string(),
                    c[2].trim().to_string(),
                    PAGE_URL.to_string() + c[0],
                )
            })
            .collect();

        Ok(())
    }
}

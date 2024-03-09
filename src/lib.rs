use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, error::Error, fs::File, io::BufReader};

const PAGE_URL: &str = "http://taharut.org/imo/";

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub name: String,
    pub author: String,
    pub url: String,
}

impl Exercise {
    pub fn new(name: String, author: String, url: String) -> Self {
        Exercise { name, author, url }
    }
}

pub struct Exercises {
    file: File,
}

impl Exercises {
    pub fn new(file: File) -> Self {
        Self { file }
    }

    pub fn get(&self) -> HashSet<Exercise> {
        let reader = BufReader::new(&self.file);
        serde_json::from_reader(reader).unwrap_or_default()
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let res = ureq::get(PAGE_URL).call()?.into_string()?;

        let re = Regex::new(r#"<a href = "([^"]+)">([^<]+)</a>([^<]+)<br>"#)?;

        serde_json::to_writer_pretty(
            &self.file,
            &re.captures_iter(&res)
                .map(|c| {
                    let c = c.extract::<3>().1;
                    Exercise::new(
                        c[1].trim().to_string(),
                        c[2].trim().to_string(),
                        PAGE_URL.to_string() + c[0],
                    )
                })
                .collect::<HashSet<Exercise>>(),
        )?;

        Ok(())
    }
}

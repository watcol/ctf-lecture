use std::{fs::File, io::Read, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct List {
    pub title: String,
    pub problems: Vec<Problem>,
}

#[derive(Deserialize)]
pub struct Problem {
    pub name: String,
    pub title: String,
}

impl List {
    pub fn from_toml<P: AsRef<Path>>(toml: P) -> anyhow::Result<Self> {
        let mut file = File::open(toml)?;
        let mut toml = String::new();
        file.read_to_string(&mut toml)?;
        Ok(toml::de::from_str(&toml)?)
    }
}

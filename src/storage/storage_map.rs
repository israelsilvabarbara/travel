use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageMap {
    map: HashMap<String, PathBuf>,
    file_path: PathBuf,
}

impl StorageMap {
    pub fn new(file_path: PathBuf) -> io::Result<Self> {
        let mut map = HashMap::new();

        if file_path.exists() {
            let mut file = File::open(&file_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            map = serde_json::from_str(&contents)?;
        }

        Ok(Self { map, file_path })
    }

    pub fn insert(&mut self, key: String, value: PathBuf) -> io::Result<()> {
        self.map.insert(key, value);
        self.save()
    }

    pub fn get(&self, key: &str) -> Option<&PathBuf> {
        self.map.get(key)
    }

    pub fn remove(&mut self, key: &str) -> io::Result<()> {
        self.map.remove(key);
        self.save()
    }

    fn save(&self) -> io::Result<()> {
        let contents = serde_json::to_string(&self.map)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(contents.as_bytes())
    }
}

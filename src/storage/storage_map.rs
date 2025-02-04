use core::panic;
use std::path::PathBuf;
use std::collections::HashMap;
use std::io;
use serde::{Deserialize, Serialize};

use super::fs::{read_from_file, write_to_file};

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageMap {
    map: HashMap<String, PathBuf>,
}

impl StorageMap {
    pub fn new() -> Self {
        let content = read_from_file();

        if content.is_err() {
            panic!("Error reading data: {}", content.err().unwrap());
        }
        
        let map = serde_json::from_str(&content.unwrap());

        if map.is_err() {
            panic!("Error deserializing data: {}", map.err().unwrap());
        }

        StorageMap {
            map: map.unwrap(),
        }
    }

    pub fn is_empty(&self) -> bool{
        self.map.is_empty()
    }

    pub fn insert(&mut self, key: String, value: PathBuf) {
        self.map.insert(key, value);
        match self.save(){
            Ok(_) => {},
            Err(e) => println!("Error saving data: {}", e),
        }
    }

    pub fn get(&self, key: &str) -> Option<&PathBuf> {
        self.map.get(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn remove(&mut self, key: &str) {
        self.map.remove(key);
        if let Err(e) = self.save() {
            panic!("Could not update file. Error : {e}");
        }
    }

    pub fn clear(&mut self){
        self.map.clear();
        match self.save(){
            Ok(_) => {},
            Err(e) => println!("Error saving data: {}", e),
        }
    }

    fn save(&self) -> io::Result<()> {
        let data = serde_json::to_string(&self.map)?;

        println!("Debbuging data in save/storage_map.rs");
        println!("Storage data before saving: {}", data);
        
        if let Err(e) = write_to_file(&data) {
            println!("Error saving data: {}", e);
            return Err(e);
        }

        Ok(())
    }



    pub fn iter(&self) -> StorageMapIter {
        StorageMapIter {
            iter: self.map.iter(),
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.map.keys()
    }
    
}

pub struct StorageMapIter<'a> {
    iter: std::collections::hash_map::Iter<'a, String, PathBuf>,
}

impl<'a> Iterator for StorageMapIter<'a> {
    type Item = (&'a String, &'a PathBuf);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_map_insert_and_get() {
        let mut storage = StorageMap::new();
        storage.insert("1".to_string(), PathBuf::from("apple"));
        assert_eq!(storage.get("1"), Some(&PathBuf::from("apple")));
    }

    #[test]
    fn test_storage_map_remove() {
        let mut storage = StorageMap::new();
        storage.insert("1".to_string(), PathBuf::from("apple"));
        storage.remove("1");
        assert!(storage.get("1").is_none());
    }

    #[test]
    fn test_storage_map_iter() {
        let mut storage = StorageMap::new();
        storage.insert("1".to_string(), PathBuf::from("apple"));
        storage.insert("2".to_string(), PathBuf::from("banana"));
        let iter = storage.iter();
        let items: Vec<_> = iter.collect();
        assert_eq!(items.len(), 2);
        assert!(items.contains(&(&"1".to_string(), &PathBuf::from("apple"))));
        assert!(items.contains(&(&"2".to_string(), &PathBuf::from("banana"))));
    }

    #[test]
    fn test_storage_map_keys() {
        let mut storage = StorageMap::new();
        storage.insert("1".to_string(), PathBuf::from("apple"));
        storage.insert("2".to_string(), PathBuf::from("banana"));
        let keys: Vec<_> = storage.keys().collect();
        assert_eq!(keys, vec![&"1".to_string(), &"2".to_string()]);
    }
}

use std::path::PathBuf;

use crate::storage::storage_map::StorageMap;

pub fn execute(storage: &mut StorageMap, id: String, path: PathBuf) {
    
    storage.insert(id.clone(), path.clone());
    println!("Adding pinpoint with ID: {}, Path: {:?}", id, path);
}

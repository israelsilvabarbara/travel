use crate::storage::storage_map::StorageMap;
use crate::utils::terminal::open_terminal;
use crate::utils::fuzzy_matcher;

pub fn execute(storage: &StorageMap, path: String) {
    
    if storage.contains_key(&path) {
        open_terminal(&path);    
    }else {
        println!("Pinpoint with ID: {} not found", path);
        let suggestions:Vec<String> = fuzzy_matcher::suggest_similar(&path, storage.keys());
        if suggestions.len() > 0 {
            println!("Did you mean: {}", suggestions.join(", "));
        }
    }
}

use crate::utils::fuzzy_matcher;
use crate::utils::terminal::open_terminal;
use crate::storage::storage_map::StorageMap;

pub fn execute(storage: &StorageMap, pinpoint: String, key: String, auto: bool) {
    if auto {
        if let Some(path) = storage.get(&pinpoint) {
            open_terminal(&path.to_string_lossy());    
        }else {
            println!("Pinpoint with ID: {} not found", pinpoint);
            let suggestions:Vec<String> = fuzzy_matcher::suggest_similar(&pinpoint, storage.keys());
            if suggestions.len() > 0 {
                println!("Did you mean: {}", suggestions.join(", "));
            }
        }
    } else {
        println!("Finding: {}, with key: {}", pinpoint, key);
    }
}

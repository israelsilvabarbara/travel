use crate::storage::storage_map::StorageMap;

pub fn execute(storage: &mut StorageMap, force: bool) {
    if force == false {
        println!("Are you sure you want to clear all? (y/n)");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() != "y" {
            println!("Clearing canceled");
            return;
        }
    }    
    storage.clear();
}

use std::path::PathBuf;

use crate::storage::storage_map::StorageMap;

pub fn execute(storage: &mut StorageMap, id: String, path: PathBuf) {
    
    if storage.contains_key(&id) && !message::confirm_overwrite(&id) {
        return ;
    }
    
    storage.insert(id.clone(), path.clone());
    println!("Adding pinpoint with ID: {}, Path: {:?}", id, path);
}




mod message{
    
    pub fn confirm_overwrite(id: &str) -> bool{

        println!("Pinpoint with ID: {} already exists", id);
        println!("Are you sure you want to overwrite? (y/n)");

        let mut input = String::new();
        
        std::io::stdin().read_line(&mut input).unwrap();
        
        if input.trim() != "y" && input.trim() != "Y" {
            println!("Overwrite canceled");
            return false;
        }
        return true;
    }
}

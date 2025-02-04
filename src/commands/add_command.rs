use std::path::PathBuf;

use crate::storage::storage_map::StorageMap;

pub fn execute(storage: &mut StorageMap, id: String, path: PathBuf) {
    
    if storage.contains_key(&id) && !message::confirm_overwrite(&id) {
        return ;
    }
    
    storage.insert(id.clone(), path.clone());
    message::success(id, path);
}


mod message{
    use std::path::PathBuf;

    use crate::utils;
    use colored::Colorize;

    
    pub fn confirm_overwrite(id: &str) -> bool{
        let message = format!("Pinpoint with ID: {} already exists", id);
        
        utils::printer::print_square(vec![
            &message,
            "Are you sure you want to overwrite? (y/n)",
        ]);

        let mut input = String::new();
        
        std::io::stdin().read_line(&mut input).unwrap();
        
        if input.trim() != "y" && input.trim() != "Y" {
            println!("\tOverwrite {}!", "canceled".red());
            return false;
        }
        return true;
    }

    pub fn success( id: String, path: PathBuf ) {
        println!("{}{}{}{:?}","Adding pinpoint with ID: ".green(), id,", Path: ".green(), path);

    }
}

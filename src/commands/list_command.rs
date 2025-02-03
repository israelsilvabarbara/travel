
use crate::storage::{self, storage_map::StorageMap};

pub fn execute(storage: &StorageMap) {
    if storage.is_empty() {
        println!("Storage is empty");
        println!("Debugging Commands/list_command.rs : execute() {}", storage::fs::get_file_path().display());
    }else {
        storage.iter().for_each(|item| {
            println!("ID: {}, Path: {:?}", item.0, item.1);
        });
    }
}

use crate::storage::storage_map::StorageMap;

pub fn execute(storage: &mut StorageMap, id: String) {
    if storage.contains_key(&id) {
        println!("Are you sure you want to delete {} ? (y/n)", id);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() != "y" {
            println!("Deletion canceled");
            return;
        }
        let _ = storage.remove(&id).expect(&id);
    }else {
        println!("Pinpoint with ID: {} not found", id);
    }
    
}

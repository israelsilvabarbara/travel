use crate::storage::storage_map::StorageMap;

pub fn execute(storage: &StorageMap) {

    storage.iter().for_each(|item| {
        println!("ID: {}, Path: {:?}", item.0, item.1);
    });
}

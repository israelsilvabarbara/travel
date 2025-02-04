use crate::storage::storage_map::StorageMap;

pub fn execute(storage: &mut StorageMap, id: String) {
    if storage.contains_key(&id) == false  {
        message::error_not_found(&id);
        return;
    }
        
    if message::confirm_deletion(&id) == false {
        message::interrupted();
        return;
    }

    storage.remove(&id);
    message::success(&id);
    
}

mod message {
    use colored::Colorize;
    use crate::utils::printer;


    pub fn confirm_deletion(id: &str) -> bool{
        let confirmation = format!("Are you sure you want to delete {} ? (y/n)",id);
        
        printer::print_square(vec![
            "Deletion of  items is a process that cannot be undone.",
            &confirmation,
        ]);

        let mut input = String::new();
        
        std::io::stdin().read_line(&mut input).unwrap();
        
        if input.trim() != "y" && input.trim() != "Y" {
            return false;
        }
        return true;
    }

    pub fn error_not_found(id: &str) {
        println!("{}{}{}","Pinpoint with ID: ".red(), id, " not found".red());
        
    }


    pub fn success(id: &str){
        println!("{}{}{}", "Pinpoint with ID: ", id, "deleted".green());
    }

    pub fn interrupted() {
        println!("\tDeletion {}!", "canceled".red());
    }
}

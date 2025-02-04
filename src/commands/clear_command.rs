use crate::storage::storage_map::StorageMap;
use colored::Colorize;

pub fn execute(storage: &mut StorageMap, force: bool) {
    if force == false && message::confirm_deletion() == false {   
        return;
    }    

    storage.clear();
    message::success();
}


mod message {
    use colored::Colorize;
    use crate::utils::printer;


    pub fn confirm_deletion() -> bool{
        
        
        printer::print_square(vec![
            "Deletion of all items is a process that cannot be undone.",
            "Are you sure you want to overwrite? (y/n)",
        ]);

        let mut input = String::new();
        
        std::io::stdin().read_line(&mut input).unwrap();
        
        if input.trim() != "y" && input.trim() != "Y" {
            println!("\tClearing {}!", "canceled".red());
            return false;
        }
        return true;
    }

    pub fn success(){
        println!("{}","Memory cleared out!".green());
    }
}
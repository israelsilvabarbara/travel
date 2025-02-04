use crate::utils::{ fuzzy_matcher, finder } ;
use crate::storage::storage_map::StorageMap;


pub fn execute(storage: &StorageMap, pinpoint: String, key: String) {
    
    if let Some(path) = storage.get(&pinpoint) {
       match finder::find_in_directory(path, &key) {
            Ok(list) => {
                message::success(list);
            },
            Err(e) => {
                message::error_panic(e);
            }
                
       }    
    }else {
        let suggestions:Vec<String> = fuzzy_matcher::suggest_similar(&pinpoint, storage.keys());
        if suggestions.len() > 0 {
            message::show_suggestions(&pinpoint,suggestions);
        }
    }
}



mod message{
    use std::io::{self, Error};

    use crate::utils::printer;

    pub fn show_suggestions( pinpoint:&str, suggestions:Vec<String>) {
        let error = format!("Pinpoint with ID: {} not found", pinpoint);

        let mut messages:Vec<String> = vec![
            error,
            String::from("Did you mean"),
        ];

        for suggestion in suggestions {
            let formated = format!("\t-{}",suggestion);
            messages.push( formated);
        }

        printer::print_square(messages.iter().map(|s| s.as_str()).collect());
    }

    pub fn success(list : Vec<String>) {
        let mut messages = vec![];

        for item in list {
            let formated = format!("-{}", item);
            messages.push( formated);
        }
        printer::print_square(messages.iter().map(|i| i.as_str()).collect());

    }

    pub fn error_panic(e: io::Error) {
        
        printer::print_square( e.to_string().split("\n").collect());
    }
}
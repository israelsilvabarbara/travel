
use std::path::PathBuf;

use crate::storage::{self, storage_map::StorageMap};

pub fn execute(storage: &StorageMap) {
    if storage.is_empty() {
        message::is_empty();
        return;
    }
    
    message::list_items( storage.iter().collect() );
}



mod message {
    use std::{io::Error, path::PathBuf};
    use crate::utils::printer;
    use colored::Colorize;
    
    pub fn is_empty() {
        let header = vec!["List of Pinpoints"];
        let messages = vec![
            "",
            "",
        ];
        printer::print_square_with_header(header, messages);
    }

    pub fn list_items(items: Vec< (&String, &PathBuf) >) {
        let header = vec!["List of Pinpoints"];

        let mut messages:Vec<String> = vec![];
        for item in items {
            let formated = format!("- {} : {}", item.0, item.1.to_string_lossy().truecolor(128, 128, 128));
            messages.push( formated);
        }
        printer::print_square_with_header(header, messages.iter().map(|i| i.as_str()).collect());
    }
}
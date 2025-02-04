use crate::storage::storage_map::StorageMap;
use crate::utils::terminal::open_terminal;
use crate::utils::fuzzy_matcher;

pub fn execute(storage: &StorageMap, path: String) {
    
    if storage.contains_key(&path) {
        open_terminal(&path);
        return;    
    }
    
    let suggestions:Vec<String> = fuzzy_matcher::suggest_similar(&path, storage.keys());
    if suggestions.len() > 0 {
        message::show_suggestions(suggestions);
        return;
    }

    message::not_found();
    
}


mod message {
    use crate::utils::printer;
    use colored::Colorize;

    pub fn not_found() {
        let binding = "Path not found".red();
        let message:Vec<&str> = vec![
            &binding,
        ];
        printer::print_square(message);
    }

    pub fn show_suggestions(suggestions:Vec<String>) {
        let header:Vec<&str> = vec![
            "Did you mean:"];

        printer::print_square_with_header(header, suggestions.iter().map(|s| s.as_str()).collect());
    }

}
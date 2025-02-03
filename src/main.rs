
mod args;
mod dispatcher;
mod commands;
mod manager;
mod storage;
mod utils; 

fn main() {
    //manager::init();

    let id = 13;
    let header = vec![
        "Travel app",
        "Version: 1.0.0"
    ];

    let overide_question = format!("the id: {} already exists in the database", id);

    let messages = vec![
        "This is a message test",
        &overide_question,
        "do you want to overid it?",
    ];

    utils::printer::print_square(messages.clone());
    println!("");
    println!("");
    utils::printer::print_square_with_header(header, messages);
}


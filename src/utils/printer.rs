use std::cmp;
use colored::{self, Colorize};

pub fn print_square(messages: Vec<&str>) {
    // Find the length of the longest message
    let mut max_length = messages.iter().map(|s| s.len()).max().unwrap_or(0);
    let prefix_message = "| ".truecolor(60,60,60);
    let sufix_message  = " |".truecolor(60,60,60);
    let icon_plus = "+".truecolor(128, 128, 128);
    let tab_size = 6;

    max_length = max_length + prefix_message.len() + sufix_message.len();
    let border_vertical = format!("{}{}{}",icon_plus, "—".repeat(max_length + 2 + tab_size).truecolor(60,60,60), icon_plus);
    
    println!("{}",border_vertical);
    for message in messages {
        println!("{}\t{: <width$}{}",prefix_message, message, sufix_message, width = max_length);
    }
    println!("{}", border_vertical);
}


pub fn print_square_with_header(header: Vec<&str>,messages: Vec<&str>) {
    // Find the length of the longest message
    let message_max_length = messages.iter().map(|s| s.len()).max().unwrap_or(0);
    let header_max_length = header.iter().map(|s| s.len()).max().unwrap_or(0);

    let mut max_length = cmp::max(header_max_length, message_max_length);
    let prefix_message = "| ".truecolor(60,60,60);
    let sufix_message  = " |".truecolor(60,60,60);
    let icon_plus = "+".truecolor(128, 128, 128);
    let tab_size = 6;


    max_length = max_length + prefix_message.len() + sufix_message.len();

    let border_vertical = format!("{}{}{}",icon_plus, "—".repeat(max_length + 2 + tab_size).truecolor(60,60,60), icon_plus);
        
    println!("{}", border_vertical);
    for header in header {
        println!("{}{: <width$}{}", prefix_message, header, sufix_message, width = max_length + tab_size);
    }
    println!("{}", border_vertical);


    // Print each message with padding
    for message in messages {
        println!("{}\t{: <width$}{}",prefix_message, message, sufix_message, width = max_length);
    }
    println!("{}", border_vertical);
    
}

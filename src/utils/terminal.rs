use std::process::Command;
use super::terminal_type::{ TerminalType, detect_terminal };


pub fn open_terminal(path: &str) {
 
    match detect_terminal() {
        TerminalType::GnomeTerminal => open_gnome_tab(path),
        TerminalType::Konsole => open_konsole_tab(path),
        TerminalType::Xterm => open_xterm(path),
        TerminalType::Unknown => println!("Unknown terminal"),
    }
}



fn open_gnome_tab(path: &str) {
    Command::new("gnome-terminal")
        .arg("--tab")
        .arg("--working-directory")
        .arg(path)
        .spawn()
        .expect("failed to start new terminal tab");
}

fn open_konsole_tab(path: &str) {
    Command::new("konsole")
        .arg("--new-tab")
        .arg("--workdir")
        .arg(path)
        .spawn()
        .expect("failed to start new terminal tab");
}

fn open_xterm(path: &str) {
    Command::new("xterm")
    .arg("-e")
    .arg(format!("cd {}; bash", path))
    .spawn()
    .expect("failed to start new terminal");
}



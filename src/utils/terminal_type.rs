use std::process::Command;

pub enum TerminalType {
    GnomeTerminal,
    Konsole,
    Xterm,
    Unknown,
}

pub fn detect_terminal() -> TerminalType {
    let terminals = ["gnome-terminal", "xterm", "konsole", ];

    for term in &terminals {
        if let Ok(output) = Command::new("which").arg(term).output() {
            if !output.stdout.is_empty() {
                return match *term {
                    "gnome-terminal" => TerminalType::GnomeTerminal,
                    "konsole" => TerminalType::Konsole,
                    "xterm" => TerminalType::Xterm,
                    _ => TerminalType::Unknown,
                };
            }
        }
    }

    TerminalType::Unknown
}



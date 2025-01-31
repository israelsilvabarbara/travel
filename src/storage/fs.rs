use std::env;
use std::fs::{File, OpenOptions};
use std::io::{ self, Read, Write};
use std::path::PathBuf;


// Function to read data from a file
pub fn read_from_file() -> io::Result<String> {
    let file_path = get_file_path();
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// Function to write data to a file
pub fn write_to_file(data: &str) -> io::Result<()> {
    let file_path = get_file_path();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

// Private function to get the file path based on the OS
fn get_file_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        FILE_PATH
    }
    
    #[cfg(target_os = "linux")]
    {
        if let Ok(snap_data) = env::var("SNAP_DATA") {
            PathBuf::from(snap_data).join("data.json")
        } else {
            let mut path = dirs::home_dir().expect("Could not find home directory");
            path.push(".travel");
            path.push("data.json");
            path
        }
    }
}

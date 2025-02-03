use std::env;
use std::fs::{File, OpenOptions};
use std::io::{ self, Read, Write};
use std::path::PathBuf;


// Function to read data from a file
pub fn read_from_file() -> io::Result<String> {
    let file_path = get_file_path();
    println!("Debugging Storage/fs/read_from_file.rs");
    match File::open(file_path.clone()) {
        Ok(mut file) => { 
            println!("\t File:open() => Ok()");
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            println!("\t content: {}", content);
            return Ok(content) 
        },
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            if let Err(e) = create_file(&file_path){
                println!("Error creating file: {}", e);
                return Err(e);
            }
            Ok("{}".to_string())
        },
        Err(e) => {
            println!("Error opening file: {}", e);
            Err(e)
        }
    }
}

// Function to write data to a file
pub fn write_to_file(data: &str) -> io::Result<()> {
    let file_path = get_file_path();

    let file = OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .create(true)
                            .open(file_path);  

    match file {
        Ok(mut file) => {
            file.write_all(data.as_bytes())?;
            Ok(())
        },
        Err(e) => {
            println!("Error opening file: {}", e);
            Err(e)
        }
    }      
}


fn create_file(file_path: &PathBuf) -> io::Result<()> {
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)?;
    // Optionally, write initial data to the file
    file.write_all(b"{}")?;
    Ok(())
}

// Private function to get the file path based on the OS
pub fn get_file_path() -> PathBuf {
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


#[cfg(test)]
mod tests {
    use super::*;


    #[test]

    fn test_read_from_file() {
        let result = read_from_file();
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_to_file() {
        let data = "Hello, world!";
        let result = write_to_file(data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_file_path() {
        let path = get_file_path();
        println!("################### Path: {}", path.display());
        assert!(path.exists()); 
    }
}





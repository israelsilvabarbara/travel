use std::{io::Error, path::PathBuf, process::Command};

pub fn find_in_directory(path:&PathBuf ,  search:&str) -> Result<Vec<String>, Error> {
    let handler = Command::new("find")
        .arg(path)
        .arg("-name")
        .arg(search)
        .output();

    match handler{
        Ok(output) => {
            let mut list:Vec<String>  = vec![];
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                list = result.lines().map(|line| line.to_string()).collect();
            }
            Ok(list)
        },

        Err(e) => {
               Err(e)
        }
    }
}
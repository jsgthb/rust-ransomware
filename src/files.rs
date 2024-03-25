use std::{fs, io::Error};

pub fn get_filepaths_in_cwd() -> Result<Vec<String>, Error> {
    // Create empty vector
    let mut directories: Vec<String> = Vec::new();
    // Read files in current directory
    match fs::read_dir("./") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        // Add path to string vector
                        directories.push(entry.path().into_os_string().into_string().expect("Path string conversion failed"))
                    }
                    Err(e) => return Err(e)
                }
            }
        }
        Err(e) => return Err(e)
    }
    return Ok(directories);
}
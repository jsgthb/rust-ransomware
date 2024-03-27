use std::{fs::{self, File}, io::{Error, Write}};

pub fn get_filepaths_in_cwd() -> Result<Vec<String>, Error> {
    // Create empty vector
    let mut directories: Vec<String> = Vec::new();
    // Read files in current directory
    let dir = fs::read_dir("./").expect("Failed to read directory");
       
    for entry in dir {
        let entry_unwrapped = entry.expect("Could not read path");
        let filename = entry_unwrapped.file_name().into_string().expect("Could not convert string");
        let path = entry_unwrapped.path();
        if path.is_file() {
            directories.push(filename)
        }
    }      
    return Ok(directories);
}

pub fn read_file_bytes(filepath: &str) -> Result<Vec<u8>, Error> {
    let file_bytes = fs::read(filepath)?;
    Ok(file_bytes)
}

pub fn save_file_bytes(filepath: &str, file_data: Vec<u8>) -> Result<(), Error> {
    let mut file = File::create(filepath)?;
    file.write_all(&file_data)?;
    Ok(())
}

pub fn delete_file(filepath: &str) -> Result<(), Error> {
    fs::remove_file(filepath)?;
    Ok(())
}
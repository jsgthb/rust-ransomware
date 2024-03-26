use std::{collections::btree_map::Entry, fs::{self, File}, io::{BufReader, Error, Read, Write}};

pub fn get_filepaths_in_cwd() -> Result<Vec<String>, Error> {
    // Create empty vector
    let mut directories: Vec<String> = Vec::new();
    // Read files in current directory
    let dir = fs::read_dir("./").expect("Failed to read directory");
       
    for entry in dir {
        let path = entry.expect("Could not read path").path();
        if path.is_file() {
            directories.push(path.into_os_string().into_string().expect("Could not convert PathBuf to String"))
        }
    }      
    return Ok(directories);
}

pub fn read_file_bytes(filepath: &str) -> Result<Vec<u8>, Error> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    // Read file
    match reader.read_to_end(&mut buffer) {
        Ok(_) => return Ok(buffer),
        Err(e) => Err(e),
    }
}

pub fn save_file_bytes(filepath: &str, file_data: Vec<u8>) -> Result<(), Error> {
    let encrypted_filepath = format!("{}.enc", filepath);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(encrypted_filepath)?;

    match file.write_all(&file_data) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
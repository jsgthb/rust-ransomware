use std::{fs::{self, File}, io::{BufReader, Error, Read, Write}};

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
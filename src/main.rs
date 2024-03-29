use std::env;

use inquire::{Confirm, InquireError, Select, Text};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

mod files;
mod encrypt;

fn main() {
    // Select encryption ot decryption
    let options: Vec<&str> = vec!["Encrypt", "Decrypt"];
    let answer: Result<&str, InquireError> = Select::new("Encrypt or decrypt files?", options).prompt();
    match answer {
        Ok(choice) => {
            if choice == "Encrypt" {
                encrypt_files()
            } else {
                decrypt_files()
            }
        }
        Err(_) => println!("Error with questionaire"),
    }
}

fn encrypt_files() {
    // Get confirmation from user
    let confirmation = Confirm::new("Are you sure you want to encrypt the files in the current directory?")
        .with_default(false)
        .with_help_message("You are responsible for your own actions")
        .prompt()
        .expect("Error with questionaire");

    // Exit if confirmation is false
    if confirmation == false {
        println!("Exiting program");
        std::process::exit(0)
    }

    // Get files in current directory
    let files = files::get_filepaths_in_cwd().expect("Files could not be parsed");
    println!("Found {} files in current directory", &files.len());

    // Generate random password for encryption key
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    let salt = encrypt::generate_salt();
    let encryption_key = encrypt::generate_encryption_key(&password, &salt).expect("Could not generate encryption key");
    let salt_and_password = format!("{} {}", password, salt);
    files::save_file_bytes("password.txt", salt_and_password.as_bytes()).expect("Saving password file failed");
    // Remove password and salt from memory
    drop(password);
    drop(salt);
    drop(salt_and_password);

    // Loop through files and encrypt them
    for file in files {
        // Skip iteration if file is current executable
        let current_exe = env::current_exe().expect("Could not get current exe path");
        let exe_filename = current_exe.file_name().expect("Could not get filename");
        if exe_filename.to_str() == Some(file.as_str()) {
            println!("Skipping executable");
            continue;
        }
        
        // Encrypt file
        let ciphertext = encrypt::encrypt_file(&file, encryption_key).expect("Encryption failed");
        let encrypted_filepath = format!("{}.enc", file);
        match files::save_file_bytes(&encrypted_filepath, &ciphertext) {
            Ok(_) => {
                println!("Encrypted file {}", &file);
                match files::delete_file(&file) {
                    Ok(_) => println!("Deleted original file"),
                    Err(e) => println!("Could not delete original file ({})", e),
                }
            },
            Err(e) => {
                println!("Error encrypting file {} ({})", &file, e)
            }
        }
    }
}

fn decrypt_files() {
    // Get password and salt to generate key
    let password = Text::new("Enter plaintext password").prompt().expect("Error with questionaire");
    let salt = Text::new("Enter salt").prompt().expect("Error with questionaire");
    let encryption_key = encrypt::generate_encryption_key(&password, &salt).expect("Could not generate encryption key");

    let files = files::get_filepaths_in_cwd().expect("Files could not be parsed");
    println!("Found {} files in current directory", &files.len());
    for file in files {
        // Check if file is larger than single character with file extension
        if file.len() < 5 {
            continue;
        }

        // Check if file is encrypted
        let file_extension = {
            let split_pos = file.char_indices().nth_back(3).expect("Could not get file extension").0;
            &file[split_pos..]
        };
        if file_extension == ".enc" {
            println!("Encrypted file found")
        }

        // Decrypt file
        let file_bytes = files::read_file_bytes(&file).expect("Could not read file");
        todo!()
    }
}
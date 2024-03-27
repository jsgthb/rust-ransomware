use inquire::{Confirm, InquireError, Select};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::{encrypt::{encrypt_file, generate_encryption_key}, files::save_file_bytes};

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
    let encryption_key = generate_encryption_key(&password).expect("Could not generate encryption key");
    save_file_bytes("password.txt", password.as_bytes().to_vec()).expect("Saving password file failed");

    // Loop through files and encrypt them
    for file in files {
        let ciphertext = encrypt_file(&file, encryption_key).expect("Encryption failed");
        let encrypted_filepath = format!("{}.enc", file);
        match save_file_bytes(&encrypted_filepath, ciphertext) {
            Ok(_) => {
                println!("Encrypted file {}", &file)
            },
            Err(e) => {
                println!("Error encrypting file {} ({})", &file, e)
            }
        }
    }
}

fn decrypt_files() {
    todo!()
}
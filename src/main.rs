use inquire::Confirm;

mod files;

fn main() {
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
}

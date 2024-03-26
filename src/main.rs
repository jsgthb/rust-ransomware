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
}

mod files;

fn main() {
    match files::get_filepaths_in_cwd() {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("{:?}", e)
    }
}

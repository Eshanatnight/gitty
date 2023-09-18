mod user;
mod file_utils;

use clap::Parser;

use crate::file_utils::{recursive_serach, get_dotfiles_path, add_new_slice_elems_to_file};

#[derive(Debug, Parser)]
#[command(name = "gitty")]
struct Cli {
    add: bool, // should pass a path change the type to String
}



fn scan(path: &str) {
    println!("Found Folders: ");
    let repos = recursive_serach(path);
    let file_path = get_dotfiles_path();
    add_new_slice_elems_to_file(&file_path, &repos);
    println!("Add Successful");
}

fn stats(email: &str) {
    println!("Stats for {}", email);
}

fn main() {
    let folder: &str = "./";
    let email: &str = "eshan2001221@gmail.com";

    if folder != "" {
        scan(folder);
        return;
    }

    stats(email)
}

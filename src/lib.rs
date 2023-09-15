use std::{env, error::Error, path::PathBuf, str::FromStr, fs};

mod errors;
mod file_node;

pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let path = match args.get(1) {
        // failing any of these is a failure to run rls
        Some(path) => PathBuf::from_str(path)?,
        None => env::current_dir()?,
    };

    fs::read_dir(path);

    Ok(())
}

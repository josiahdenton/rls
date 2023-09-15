use std::{env, error::Error, fs, path::PathBuf, str::FromStr};

mod file_node;

use file_node::FileNode;

pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let path = match args.get(1) {
        // failing any of these is a failure to run rls
        Some(path) => PathBuf::from_str(path)?,
        None => env::current_dir()?,
    };

    // read paths and map into a vec of file nodes
    // any error that occurs here will need to be handled by the caller
    let mut file_nodes = fs::read_dir(path)?
        .map(|result| {
            result.map(|entry| {
                let name = entry.file_name();
                entry.metadata().map_or(None, move |meta| {
                    Some(FileNode::new(name, meta.len(), meta.is_dir()))
                })
            })
        })
        .collect::<Result<Vec<Option<FileNode>>, std::io::Error>>()?;


    // sort for consistent view
    file_nodes.sort();

    file_nodes.iter().for_each(|node| {
        if let Some(node) = node {
            println!("{}", node);
        }
    });

    Ok(())
}

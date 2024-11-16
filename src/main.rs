use std::io::prelude::*;
use std::{fs, io, path::Path};

fn handle_path(path: &Path) {
    if path.is_dir() {
        println!("Path Valid:");
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                match entry {
                    Ok(item) => {
                        let item_path = item.path();
                        // Print the type of the item (directory, file, or unknown)
                        if item_path.is_dir() {
                            println!("Directory: {}", item_path.display());
                            handle_path(&item_path); // Recursively handle subdirectories
                        } else if item_path.is_file() {
                            println!("File---------: {}", item_path.display());

                            // Cek apakah file memiliki ekstensi .js
                            if item_path.extension().and_then(|e| e.to_str()) == Some("js") {
                                let mut new_item_path = item_path.clone();
                                new_item_path.set_extension("jsx");

                                match fs::rename(&item_path, &new_item_path) {
                                    Ok(_) => println!(
                                        "Renamed From {:?} to {:?}",
                                        item_path.display(),
                                        new_item_path.display()
                                    ),
                                    Err(e) => eprintln!("Error renaming file: {}", e),
                                }
                            }
                        } else {
                            println!("Unknown: {}", item_path.display());
                        }
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        } else {
            eprintln!("Failed to read the directory contents");
        }
    } else {
        println!("Directory not found");
    }
}

fn main() {
    // Read the path from standard input
    println!("Change your .js => .jsx");
    io::stdout().flush().ok().expect("Could not flush stdout");
    println!("Enter the path:");
    let mut path_name = String::new();
    io::stdin()
        .read_line(&mut path_name)
        .expect("Failed to read line");

    // Trim the input path
    let path_name = path_name.trim();

    // Create a Path from the input string
    let path = Path::new(path_name);

    handle_path(path);
}

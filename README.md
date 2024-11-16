# Directory and File Handler

This Rust program reads a directory path from user input and performs several tasks on the contents of the specified directory. The program can:

- Print the type of each entry (directory or file).
- Recursively explore subdirectories.
- Identify `.js` files and rename them to `.jsx` files.

## Features

1. **Directory Traversal**: 
   - If the given path is a directory, it reads and traverses all subdirectories.
   
2. **File Handling**:
   - Each file is identified and, if it has a `.js` extension, it is renamed to `.jsx`.
   
3. **Error Handling**:
   - Handles errors during directory reading and file renaming operations.

## Usage

1. **Run the Program**:
   ```bash
   cargo run
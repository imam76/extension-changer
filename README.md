# Directory and File Renamer

This Rust program navigates through directories and files, printing out their paths and renaming JavaScript files (with `.js` extension) to JSX files (with `.jsx` extension). It handles subdirectories recursively and is designed to operate from a given directory path input by the user.

## Features

- **Directory Traversal:** Navigates through the given directory path recursively.
- **File Detection:** Identifies files and directories, printing each item path.
- **File Renaming:** Changes the file extension of `.js` files to `.jsx`.
- **Error Handling:** Handles various IO errors, such as inaccessible directories and failed file renaming.

## Getting Started

### Prerequisites

To build and run this program, you will need:
- [Rust](https://www.rust-lang.org/tools/install) installed on your system.

### Running the Program

1. Clone this repository or copy the source code into a local file, for example `main.rs`.
2. Open a terminal in the directory containing `main.rs`.
3. Compile the program by running:
   ```bash
   rustc main.rs

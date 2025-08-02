# why

An interpreter for an educational programming language called "why" to help beginners learn programming logic.

## Prerequisites
You need to have [Rust](https://www.rust-lang.org/tools/install) and Cargo installed on your system.

You can install Rust and Cargo by following the instructions at:  
https://www.rust-lang.org/tools/install

To verify installation, run:

```bash
rustc --version
cargo --version
```
## Installation

1. Clone the repository:

  git clone https://github.com/sharonjeyakumar/why-lang.git

2. Build the project (requires Rust and Cargo):

  cargo build --release
   
3. Add the folder containing `why.exe` (usually `target/release/`) to your system's `PATH` environment variable, so you can run `why` from anywhere.

  * On **Windows**:

    * Search for "Edit environment variables"
    * Edit the `PATH` variable and add the full path to `target\release`

  * On **Linux/macOS**:
    * Add the following line to your shell profile (e.g., `.bashrc`, `.zshrc`):

      export PATH=$PATH:/path/to/why-lang/target/release
       
    
## Usage

Run your `.why` code file with:

```bash
why <filename>
```
  
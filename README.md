# BhejDe - Simple Git Command Line Tool

## Overview

**BhejDe** (which means "Send it" in Hindi) is a simple command line tool written in Rust. It automates the process of adding all files, committing changes, and pushing them to the remote repository. This tool is designed to streamline the Git workflow, making it faster and more convenient.

## Usage

To use **BhejDe**, follow these steps:

1. Navigate to the root directory of your Git repository in the terminal.

2. Run the following command:

```bash
bhejde
```

This command will execute the tool and perform the following operations:

1. Add all files recursively to the Git repository.
2. Commit all changes with a randomly generated commit message.
3. Push the changes to the remote repository (origin main branch).

## Installation

To build and install **BhejDe**, you'll need Rust and Cargo installed on your system. If you haven't already, you can install Rust by following the instructions on the official [Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can proceed with the following steps:

1. Clone the BhejDe repository:

```bash
git clone https://github.com/lordofwizard/bhejde.git
```

2. Navigate to the project directory:

```bash
cd bhejde
```

3. Build the project using Cargo:

```bash
cargo build --release
```

4. Install the binary:

```bash
cargo install --path .
```

## Dependencies

- `names`: This dependency is used to generate random commit messages. You can find more information about this crate [here](https://crates.io/crates/names).

- `std::os`: This module is part of the Rust standard library and is used for interacting with the operating system. It is used in this project for handling process exit codes.

## License

This project is released under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

If you'd like to contribute to this project, feel free to open an issue or submit a pull request on the [GitHub repository](https://github.com/lordofwizard/bhejde).

## Acknowledgments

Special thanks to [lordofwizard][https://github.com/lordofwizard] contributors who have created this awesome project.

---


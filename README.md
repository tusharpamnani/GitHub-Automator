# GitHub Automator

The GitHub Automator is a Rust-based tool designed to streamline and automate common Git operations on GitHub repositories. Whether you're adding, committing, and pushing changes, generating commit messages, or managing pull requests, this tool simplifies the process, saving you time and effort.

## Features

- **Automated Git Operations**: Perform common Git operations, such as adding files, committing changes, and pushing commits, with ease and efficiency.
- **Commit Message Generation**: Generate meaningful commit messages automatically, eliminating the need for manual entry and ensuring consistency across commits.
- **GitHub Integration**: Seamlessly integrate with GitHub repositories, enabling smooth interaction with remote repositories and pull requests.
- **Customization Options**: Tailor the tool to your needs with customizable settings for commit message generation, branch management, and more.
- **User-Friendly Interface**: Enjoy a user-friendly command-line interface that makes Git operations intuitive and accessible.

## Getting Started

To get started with the GitHub Automator, follow these steps:

1. **Installation**: Clone the repository, compile the Rust project using Cargo, and install the tool globally.
   ```bash
   git clone https://github.com/tusharpamnani/GitHub-Automator.git
   cd GitHub-Automator
   cargo build --release
   cargo install --path .
   ```

2. **Usage**: Use the command-line interface to execute Git operations and automate your workflow.
   ```bash
   git_automate --link <repository-link> --branch <branch-name>
   ```

### Command-Line Arguments

- `--link` or `-l`: The URL of the GitHub repository. This sets the remote origin if not already set.
- `--branch` or `-b`: The branch name to push to. If not specified, defaults to `master`.
- `--help` or `-h`: Displays this help message and exits. Use this option to get information about the available command-line arguments and their usage.

## Contributing

Contributions to the GitHub Automator are welcome! If you encounter any bugs, have feature requests, or want to contribute code improvements, feel free to open an issue or submit a pull request on the project's GitHub repository.

Before contributing, please review the project's contribution guidelines and code of conduct to ensure a positive and collaborative community environment.

## License

The GitHub Automator is open-source software licensed under the [MIT License](https://opensource.org/licenses/MIT). You are free to use, modify, and distribute the tool for any purpose, subject to the terms of the license.

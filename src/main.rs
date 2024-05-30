use clap::{Arg, Command as ClapCommand};
use names::Generator;
use std::process::{Command, exit};

fn update_commit_push() {
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command");
    if !add_command.status.success() {
        eprintln!("Error: Failed to add files to the git repo");
        exit(1);
    }

    let status_command = Command::new("git")
        .arg("status")
        .output()
        .expect("Failed to execute git status command");
    if !status_command.status.success() {
        eprintln!("Error: Failed to check the status of the git repo");
        exit(1);
    }

    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&name_generator())
        .output()
        .expect("Failed to execute git commit command");
    if !commit_command.status.success() {
        eprintln!("Error: Failed to commit files to the git repo");
        exit(1);
    }

    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Failed to execute git push command");
    if !push_command.status.success() {
        eprintln!("Error: Failed to push files to the git repo");
        exit(1);
    }
    println!("Successfully added, committed, and pushed to the git repo!");
}

fn name_generator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}

fn initialize_repo(repo_link: &str) {
    let init_command = Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to execute git init command");
    if !init_command.status.success() {
        eprintln!("Error: Failed to initialize git repo");
        exit(1);
    }

    let remote_command = Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(repo_link)
        .output()
        .expect("Failed to execute git remote add command");
    if !remote_command.status.success() {
        eprintln!("Error: Failed to add remote origin");
        exit(1);
    }
    println!("Successfully initialized the repository and added remote origin.");
}

fn main() {
    let matches = ClapCommand::new("GitHub Automator")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Automates common GitHub operations")
        .arg(
            Arg::new("link")
                .short('l')
                .long("link")
                .num_args(1)
                .help("Link to the GitHub repository"),
        )
        .get_matches();

    if let Some(repo_link) = matches.get_one::<String>("link") {
        initialize_repo(repo_link);
    } else {
        update_commit_push();
    }
}

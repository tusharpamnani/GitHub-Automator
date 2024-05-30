use std::process::{Command, exit};
use clap::{App, Arg};
use names::Generator;

fn initialize_repo(repo_url: &str) {
    let init_command = Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to execute git init command");
    if !init_command.status.success() {
        eprintln!("Error: Failed to initialize git repo");
        let error_message = String::from_utf8_lossy(&init_command.stderr);
        eprintln!("Git init error: {}", error_message);
        exit(1);
    }

    let remote_command = Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(repo_url)
        .output()
        .expect("Failed to execute git remote add command");
    if !remote_command.status.success() {
        eprintln!("Error: Failed to add remote origin");
        let error_message = String::from_utf8_lossy(&remote_command.stderr);
        eprintln!("Git remote add error: {}", error_message);
        exit(1);
    }

    println!("Successfully initialized the repository and added remote origin.");
}

fn update_commit_push(branch: &str) {
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command");
    if !add_command.status.success() {
        eprintln!("Error: Failed to add files to the git repo");
        let error_message = String::from_utf8_lossy(&add_command.stderr);
        eprintln!("Git add error: {}", error_message);
        exit(1);
    }

    let status_command = Command::new("git")
        .arg("status")
        .output()
        .expect("Failed to execute git status command");
    if !status_command.status.success() {
        eprintln!("Error: Failed to check the status of the git repo");
        let error_message = String::from_utf8_lossy(&status_command.stderr);
        eprintln!("Git status error: {}", error_message);
        exit(1);
    }

    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("Failed to execute git commit command");
    if !commit_command.status.success() {
        eprintln!("Error: Failed to commit files to the git repo");
        let error_message = String::from_utf8_lossy(&commit_command.stderr);
        eprintln!("Git commit error: {}", error_message);
        exit(1);
    }

    let set_upstream_command = Command::new("git")
        .arg("branch")
        .arg("--set-upstream-to")
        .arg(format!("origin/{}", branch))
        .arg(branch)
        .output()
        .expect("Failed to execute git branch --set-upstream-to command");
    if !set_upstream_command.status.success() {
        eprintln!("Error: Failed to set upstream branch");
        let error_message = String::from_utf8_lossy(&set_upstream_command.stderr);
        eprintln!("Git set upstream error: {}", error_message);
        exit(1);
    }

    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg(branch)
        .output()
        .expect("Failed to execute git push command");
    if !push_command.status.success() {
        eprintln!("Error: Failed to push files to the git repo");
        let error_message = String::from_utf8_lossy(&push_command.stderr);
        eprintln!("Git push error: {}", error_message);
        exit(1);
    }
    println!("Successfully added, committed, and pushed to the git repo!");
}

fn name_generator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}

fn main() {
    let matches = App::new("GitHub Automator")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Automates git operations")
        .arg(
            Arg::new("link")
                .short('l')
                .long("link")
                .value_hint(clap::ValueHint::FilePath)
                .about("Sets the GitHub repository link")
                .takes_value(true),
        )
        .arg(
            Arg::new("branch")
                .short('b')
                .long("branch")
                .value_hint(clap::ValueHint::Other)
                .default_value("master")
                .about("Sets the branch name to push to")
                .takes_value(true),
        )
        .get_matches();

    let branch = matches.value_of("branch").unwrap();

    if let Some(link) = matches.value_of("link") {
        initialize_repo(link);
    }

    update_commit_push(branch);
}

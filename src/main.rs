use clap::{Arg, Command};
use names::Generator;
use std::process::{Command as ProcessCommand, exit};

fn update_commit_push(branch: Option<&str>) {
        let add_command = ProcessCommand::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command");

    if !add_command.status.success() {
        eprintln!("Error: Failed to add files to the git repo");
        exit(1);
    }

    let status_command = ProcessCommand::new("git")
        .arg("status")
        .output()
        .expect("Failed to execute git status command");

    if !status_command.status.success() {
        eprintln!("Error: Failed to check the status of the git repo");
        exit(1);
    }

    println!("{}", String::from_utf8_lossy(&status_command.stdout));

    let commit_command = ProcessCommand::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("Failed to execute git commit command");

    if !commit_command.status.success() {
        eprintln!("Error: Failed to commit files to the git repo");
        exit(1);
    }

    let branch = branch.unwrap_or("master");
    let push_command = ProcessCommand::new("git")
        .arg("push")
        .arg("origin")
        .arg(branch)
        .output()
        .expect("Failed to execute git push command");

    if !push_command.status.success() {
        eprintln!("Error: Failed to push files to the git repo");
        exit(1);
    }

    println!("Successfully added, committed and pushed to the git repo!");
}

fn name_generator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}

fn main() {
    let ascii = r#"
    
  /$$$$$$  /$$   /$$     /$$   /$$           /$$              /$$$$$$              /$$                                         /$$                        
  /$$__  $$|__/  | $$    | $$  | $$          | $$             /$$__  $$            | $$                                        | $$                        
 | $$  \__/ /$$ /$$$$$$  | $$  | $$ /$$   /$$| $$$$$$$       | $$  \ $$ /$$   /$$ /$$$$$$    /$$$$$$  /$$$$$$/$$$$   /$$$$$$  /$$$$$$    /$$$$$$   /$$$$$$ 
 | $$ /$$$$| $$|_  $$_/  | $$$$$$$$| $$  | $$| $$__  $$      | $$$$$$$$| $$  | $$|_  $$_/   /$$__  $$| $$_  $$_  $$ |____  $$|_  $$_/   /$$__  $$ /$$__  $$
 | $$|_  $$| $$  | $$    | $$__  $$| $$  | $$| $$  \ $$      | $$__  $$| $$  | $$  | $$    | $$  \ $$| $$ \ $$ \ $$  /$$$$$$$  | $$    | $$  \ $$| $$  \__/
 | $$  \ $$| $$  | $$ /$$| $$  | $$| $$  | $$| $$  | $$      | $$  | $$| $$  | $$  | $$ /$$| $$  | $$| $$ | $$ | $$ /$$__  $$  | $$ /$$| $$  | $$| $$      
 |  $$$$$$/| $$  |  $$$$/| $$  | $$|  $$$$$$/| $$$$$$$/      | $$  | $$|  $$$$$$/  |  $$$$/|  $$$$$$/| $$ | $$ | $$|  $$$$$$$  |  $$$$/|  $$$$$$/| $$      
  \______/ |__/   \___/  |__/  |__/ \______/ |_______/       |__/  |__/ \______/    \___/   \______/ |__/ |__/ |__/ \_______/   \___/   \______/ |__/      
                                                                                                                                                           
                                                                                                                                                           
                                                                                                                                                           
 
    "#;
    println!("{}", ascii);
    let matches = Command::new("GitHub Automator")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Automates GitHub operations")
        .arg(Arg::new("link")
            .short('l')
            .long("link")
            .num_args(1)
            .help("Sets the GitHub repository link"))
        .arg(Arg::new("branch")
            .short('b')
            .long("branch")
            .num_args(1)
            .help("Sets the branch name to push"))
        .get_matches();

    if let Some(link) = matches.get_one::<String>("link") {
        let init_command = ProcessCommand::new("git")
            .arg("init")
            .output()
            .expect("Failed to execute git init command");

        if !init_command.status.success() {
            eprintln!("Error: Failed to initialize git repo");
            exit(1);
        }

        let remote_command = ProcessCommand::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(link)
            .output()
            .expect("Failed to execute git remote add command");

        if !remote_command.status.success() {
            eprintln!("Error: Failed to add remote repository");
            exit(1);
        }

        let branch = matches.get_one::<String>("branch");
        update_commit_push(branch.map(|x| x.as_str()));
    } else {
        update_commit_push(None);
    }
}

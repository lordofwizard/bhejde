use std::process::{Command, exit};
use names::Generator;
use colored::*;

fn update_commit_push(commit_message: Option<&str>) {
    // Command 1: Add all files recursively to git repo
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command\nNo changes probably");

    if !add_command.status.success() {
        eprintln!("{}", "Error: Failed to add files to the git repo.\nChanges hain na? check kar.".red());
        exit(1);
    }

    // Command 2: Commit all changes
    let commit_message = match commit_message {
        Some(msg) => msg.to_string(),
        None => name_generator(),
    };

    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-a")
        .arg("-m")
        .arg(&commit_message)
        .output()
        .expect("Failed to execute git commit command");

    if !commit_command.status.success() {
        eprintln!("{}", "Error: Failed to commit changes.".red());
        exit(1);
    }

    // Command 3: Push to remote (origin main)
    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .output()
        .expect("Failed to execute git push command");

    if !push_command.status.success() {
        eprintln!("{}", "Error: Failed to push changes to remote.\nInternet Check Kar\nRepo remote check kar nahi toh".red());
        exit(1);
    }

    println!("{}", "Successfully added, committed, and pushed changes!\nKAAY VISHAY NAI NANA, pathavle warti".green());
}

fn name_generator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let commit_message = if args.len() > 1 {
        Some(&args[1])
    } else {
        None
    };
    update_commit_push(commit_message.map(|x| x.as_str()));
}

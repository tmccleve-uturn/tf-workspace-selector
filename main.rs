use inquire::{Select, InquireError};
use std::process::{Command};

fn main() {
    // Get the list of Terraform workspaces
    let output = Command::new("terraform")
        .args(&["workspace", "list", "-no-color"])
        .output()
        .expect("Failed to execute Terraform command");

    // Store the workspaces in a vector
    let workspaces = String::from_utf8_lossy(&output.stdout)
        .trim()
        .split('\n')
        .map(|workspace| workspace.trim().to_string())
        .collect::<Vec<String>>();

    // Create the selection prompt
    let selection: Result<String, InquireError>= Select::new("Select a Terraform workspace:", workspaces)
        .prompt();

    match selection {
        Ok(workspace) => {
            let _output = Command::new("terraform")
                .args(&["workspace", "select", &workspace])
                .output()
                .expect("Failed to set workspace");
        },
        Err(_) => println!("There was an error")
    }
}

# Terraform Workspace Selector

This code is a simple command-line tool written in Rust that allows you to select a Terraform workspace from a list of available workspaces. It utilizes the `inquire` and `std::process` libraries to interact with the user and execute Terraform commands.

## Prerequisites

Before running this code, make sure you have the following prerequisites installed:

- Rust programming language (https://www.rust-lang.org/tools/install)
- Terraform (https://www.terraform.io/downloads.html)

## Usage

1. Clone the repository.
2. Build and run the project:
   ```
   $ cargo build -r
   ```
3. Place the build binary from the `target/release` directory into your `$PATH`
4. The program will execute the `terraform workspace list` command to retrieve the available workspaces.
5. A selection prompt will be presented to the user, displaying the list of workspaces.
6. Select the desired workspace by entering its corresponding number.
7. The program will execute the `terraform workspace select <workspace>` command to set the selected workspace.

## Functionality

1. The program executes the `terraform workspace list -no-color` command to obtain the list of available workspaces.
2. The list is parsed and stored in a vector for displaying to the user.
3. The user is prompted to select a workspace from the displayed list.
4. If a valid workspace is selected, the program executes the `terraform workspace select <workspace>` command to set the chosen workspace.
5. If an error occurs during the execution of any command or during the selection process, an error message will be displayed.

## Note

- The code assumes that the `terraform` command is available in the system's PATH and can be executed directly.
- The code expects Terraform to be properly installed and configured on the system.
- The program uses the `inquire` library for the selection prompt and error handling.
- Error messages are displayed to the console if any errors occur during command execution or user interaction.

Feel free to modify the code according to your needs or integrate it into your own projects.

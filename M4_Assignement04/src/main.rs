use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}

fn main() {

    loop {

        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");
        println!("");
        print!("Enter your choice (0-5): ");

        io::stdout().flush().unwrap();

        let mut choice = String::new();
        
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        
        let choice = choice.trim();

        match choice {
            "1" => {
                let path = get_input("What directory path: ");
                let operation = FileOperation::List(path);
                perform_operation(operation);
            }
            "2" => {
                let path = get_input("Where is the file's path: ");
                let operation = FileOperation::Display(path);
                perform_operation(operation);
            }
            "3" => {
                let path = get_input("What file path? ");
                let content = get_input("Enter it's content: ");
                let operation = FileOperation::Create(path, content);
                perform_operation(operation);
            }
            "4" => {
                let path = get_input("Where is the file's path? ");
                let operation = FileOperation::Remove(path);
                perform_operation(operation);
            }
            "5" => {
                perform_operation(FileOperation::Pwd);
            }
            "0" => {
                println!("Successfully exited.");
                break; 
            }
            _ => println!("Invalid menu option. Try again."),
        }
    }
}

fn perform_operation(operation: FileOperation) {

    match operation {
        FileOperation::List(path) => {
            Command::new("ls").arg(&path).status().expect("Failed to execute ls");
        }
        FileOperation::Display(path) => {
            Command::new("cat").arg(&path).status().expect("Failed to execute cat");
        }
        FileOperation::Create(path, content) => {
            let command = format!("echo '{}' > {}", content, path);
            Command::new("sh").arg("-c").arg(&command).status().expect("Failed to create file");
            println!("File '{}' created successfully.", path);
        }
        FileOperation::Remove(path) => {
            Command::new("rm").arg(&path).status().expect("Failed to remove file");
            println!("File '{}' removed successfully.", path);
        }
        FileOperation::Pwd => {
            Command::new("pwd").status().expect("Failed to execute pwd");
        }
    }
}


fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}


use std::process::Command;
use std::io::{self, Write};

#[derive(Debug)]
enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}


fn perform_operation(operation: FileOperation) {
    // Implement command execution based on the operation
    match operation{
        FileOperation::List(directory_path) => {
            let result = Command::new("ls").arg(directory_path).status();
            match result{
                Ok(s) if s.success() => 
                {
                    println!("Worked!");
                },
                Ok(s) => 
                {
                    eprintln!("Somewhat worked (ls exited with status: {s})");
                },
                Err(e) => 
                {
                    eprintln!("Didnt work");
                },

            };
        }

        FileOperation::Display(file_path) => {     
            let result = Command::new("cat").arg(&file_path).status();
                match result {
                    Ok(s) if s.success() => 
                    {},
                    Ok(s) => 
                    {
                        eprintln!("cat exited with status: {s} (path: {file_path})");
                    },
                    Err(e) => 
                    {
                        eprintln!("Failed to run cat for {file_path}: {e}");
                    },
                };

        }

        FileOperation::Create(file_path, content) => {
            let cmd = format!("echo '{}' > {}", content, file_path);
                let result = Command::new("sh").arg("-c").arg(&cmd).status();

                    match result {
                        Ok(s) if s.success() => 
                        {},
                        Ok(s) => 
                        {
                            eprintln!("create failed (exit: {s}) [cmd: {}]", cmd);
                        },
                        Err(e) => 
                        {
                            eprintln!("failed to spawn shell for create: {e} [cmd: {}]", cmd);
                        },
                    };

        }

        FileOperation::Remove(file_path) => {     
            let result = Command::new("rm").arg(&file_path).status();
                match result {
                    Ok(s) if s.success() => 
                    {},
                    Ok(s) => 
                    {
                        eprintln!("rm exited with status: {s} (path: {file_path})");
                    },
                    Err(e) => 
                    {
                        eprintln!("Failed to run rm for {file_path}: {e}");
                    },
                };

        }

        FileOperation::Pwd => {   
            let result = Command::new("pwd").status();
                match result {
                    Ok(s) if s.success() => 
                    {},
                    Ok(s) => 
                    {
                        eprintln!("pwd exited with status: {s}");
                    },
                    Err(e) => 
                    {
                        eprintln!("Failed to run pwd: {e}");
                    },
                };

        }
        
        


    }
}
fn menu(){
    println!("File Operations Menu:");
    println!("1. List files in a directory");
    println!("2. Display file contents");
    println!("3. Create a new file");
    println!("4. Remove a file");
    println!("5. Print working directory");
    println!("0. Exit");
    println!(" ");
    print!("Enter your choice (0-5): ");
}


fn main() {
    println!("Welcome to the File Operation Program!!!");

 loop{
    menu();
    io::stdout().flush().ok();
    let mut choice_line = String::new();
    io::stdin().read_line(&mut choice_line).expect("Failed to read choice");
    let ans = match choice_line.trim().parse::<u32>()
    {
        Ok(n) => n,
        Err(_) => 
        {
            eprintln!("Invalid option, try again."); 
            continue; 
        }
    };
    match ans{
        0 => 
        {
            println!("Goodbye, thank you for using the application!!");
            break;
        },
        1 => 
        {
            print!("Please enter a directory path: ");
            io::stdout().flush().ok();

            let mut directory_path = String::new();
            io::stdin().read_line(&mut directory_path).expect("Can't read directory line!!");

            let trimmed = directory_path.trim();
            let dir = if trimmed.is_empty()
            {
                ".".to_string()
            }
            else
            {
                trimmed.to_string()
            };

            println!("Listing: {}", dir);
            perform_operation(FileOperation::List(dir));
        },

        2 => {
            print!("Enter file path: ");
            io::stdout().flush().ok();

            let mut file_path = String::new();
            io::stdin().read_line(&mut file_path).expect("Can't read file path");
            let file_path = file_path.trim().to_string();

            if file_path.is_empty() 
            {
                eprintln!("File path cannot be empty.");
                continue;
            }
            perform_operation(FileOperation::Display(file_path));
        },

        3 => {
            print!("Enter file path: ");
            io::stdout().flush().ok();

            let mut file_path = String::new();
            io::stdin().read_line(&mut file_path).expect("Can't read file path");
            let file_path = file_path.trim().to_string();

            if file_path.is_empty() 
            {
                eprintln!("File path cannot be empty.");
                continue;
            }

            print!("Enter content: ");
            io::stdout().flush().ok();
            let mut content = String::new();
            io::stdin().read_line(&mut content).expect("Can't read content");
            let content = content.trim().to_string();

            perform_operation(FileOperation::Create(file_path.clone(), content));
        },
        
        4 => {
            print!("Enter file path: ");
            io::stdout().flush().ok();

            let mut path_line = String::new();
            io::stdin().read_line(&mut path_line).expect("Can't read file path");
            let file_path = path_line.trim().to_string();

            if file_path.is_empty() 
            {
                eprintln!("File path cannot be empty.");
                continue;
            }

            perform_operation(FileOperation::Remove(file_path.clone()));
        },
        5 => {
            println!("Current working directory:");
            perform_operation(FileOperation::Pwd);
        },





        _ => {
            eprintln!("Not an option, please try again!");
            continue;
        }
    }



 }
}

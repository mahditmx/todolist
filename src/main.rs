use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "myapp")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Print { todo_list_name: Option<String> },
    Run,
}

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();

    // If a user wrote something that’s not a known subcommand:
    if let Some(cmd) = raw_args.get(1) {
        if !["print", "run"].contains(&cmd.as_str()) {
            return handle_unknown_command(cmd, &raw_args[2..]);
        }
    }

    // Otherwise, let clap handle parsing
    let cli = Cli::parse();
    match cli.command.unwrap_or(Commands::Print { todo_list_name: Some("default".to_string())}) {
        Commands::Run => println!("Running (default)"),
        Commands::Print { todo_list_name } => {
            let todo_list_name = todo_list_name.unwrap_or_else(|| "default".to_string());
            if todo_list_name == "default" {
                print_todo_list(&get_default_todolist());
                return;
            }
            print_todo_list(&todo_list_name);

        }
    }
}

fn handle_unknown_command(cmd: &str, args: &[String]) {
    // check if NAME is a todolist name
    // check if NAME can be a todolist
    print_todo_list(cmd);


    println!("Unknown command: {}", cmd);
    println!("Args: {:?}", args);
}


fn get_default_todolist() -> String{
    let todolist = "Xdefault".to_string();
    todolist
}

fn print_todo_list(name: &str){
    println!("TODO LIST {}", name);
}













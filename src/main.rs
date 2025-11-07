extern crate core;
use clap::{Parser, Subcommand};
mod file;
use std::io::{stdin,stdout,Write};

#[derive(Parser)]
#[command(name = "myapp")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Print   {todo_list_name: Option<String>},
    Add     {name: String, task: String}, // add task to a selected todolist.
    Create  {name: String},
    Remove  {name: String},
    Read    {name: String},
    List,
    Run,
}

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();

    // If a user wrote something that’s not a known subcommand:
    if let Some(cmd) = raw_args.get(1) {
        if !["print", "create", "list", "remove", "add", "read"].contains(&cmd.as_str()) {
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
                print_todolist(&get_default_todolist());
                return;
            }
            print_todolist(&todo_list_name);

        },
        Commands::Create { name } => {
            create_todolist(&name);
        }
        Commands::List => {
            todolist_list();
        },
        Commands::Remove { name } => {
            remove_todolist(&name);
        }
        Commands::Add {name, task} => {
            add_task_to_todolist(&name,&task);
        }
        Commands::Read { name } => {
            read_todolist(&name);
        }
    }
}

fn handle_unknown_command(cmd: &str, args: &[String]) {
    // check if NAME is a todolist name
    // check if NAME can be a todolist
    print_todolist(cmd);


    println!("Unknown command: {}", cmd);
    println!("Args: {:?}", args);
}


fn get_default_todolist() -> String{
    let todolist = "default".to_string();
    todolist
}

fn print_todolist(name: &str){
    println!("TODO LIST {}", name);
}

fn create_todolist(name: &str) {
    let r = file::create_todolist_file(&name);
    if r == false {
        print!("FAIL: cant create todolist file\n");
        return;
    }
    println!("Todolist {} created.", name);
}

fn todolist_list() {
    let todolist_vec= file::todolist_list();
    let mut counter:u16 = 0;
    for todolist in todolist_vec{
        counter += 1;
        println!("{}. {}",counter, todolist);
    }
}


fn todolist_exist(name: &str) -> bool {
    let todolist_vec= file::todolist_list();
    if !todolist_vec.contains(&name.to_string()){
        return false;
    }
    true
}

fn remove_todolist(name: &str) {
    if !todolist_exist(&name){
        println!("{} is not exist.", name);
        return;
    }



    if !ask_y_or_n(&format!("Remove {} todolist? ",name)){
        print!("Canceled by user.");
        return;
    }
    let name = &format!("{}.json", name);
    if file::remove_todolist_file(name){
        println!("{} Removed", name);
        return;
    }
    print!("FAIL: Can't remove {}", name);

}

fn input(message: &str) -> String {


    let mut input=String::new();
    print!("{}(Y/N)", message);
    let _=stdout().flush();
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    if let Some('\n')=input.chars().next_back() {
        input.pop();
    }
    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }
    input
}


fn ask_y_or_n(message: &str) -> bool {
    let output = input(message);
    if ["Yes", "Y", "y","yes"].contains(&output.as_str()){
        return true;
    }
    if ["No", "N", "n","no"].contains(&output.as_str()) {
        return false;
    }
    false
}

fn add_task_to_todolist(name: &str, task: &str) {
    if !todolist_exist(&name){
        println!("{} todolist is not exist.", name);
        return;
    }
    file::add_to_todolist(name,task);

}


fn read_todolist(name: &str) {
    if !todolist_exist(&name){
        println!("{} todolist is not exist.", name);
        return;
    }
    let todolist_json = file::read_todolist(&name);


    if let Some(obj) = todolist_json.as_object(){
        let mut  counter = 0;
        for (key,value) in obj{
            counter += 1;
            println!("{}. {}",counter,value.get("text").unwrap().as_str().unwrap());
        }
    }


}










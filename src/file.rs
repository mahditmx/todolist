use std::{env, fs::File};
use std::fs;
use std::path::Path;

pub fn create_todolist_file(name: &str) {
    let todolist_path = Path::new("./todolist");
    if !todolist_path.is_dir(){
        fs::create_dir(todolist_path).expect("Unable to create directory");
    }
    println!("path: {}", todolist_path.display());
    let todolist_path = todolist_path.join(format!("{}.json", name));
    File::create(todolist_path).expect("FAIL: create file");
}

pub fn file_list() -> Vec<String> {
    let mut file_vec: Vec<String> = Vec::new();
    let todolist_path = Path::new("./todolist");

    for file in fs::read_dir(todolist_path).unwrap(){
        let file = file.unwrap().file_name();
        file_vec.push(file.into_string().unwrap().replace(".json", ""));
    }
    file_vec

}


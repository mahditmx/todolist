use std::{fs::File};
use std::fs;
use std::path::Path;

use serde::Deserialize;

pub fn create_todolist_file(name: &str) -> bool {
    let todolist_path = Path::new("./todolist");
    if !todolist_path.is_dir(){
        fs::create_dir(todolist_path).expect("Unable to create directory.");
    }
    let todolist_path = todolist_path.join(format!("{}.json", name));
    File::create(&todolist_path).expect("FAIL: can't create file.");
    if todolist_path.exists() {
        true
    } else {
        false
    }
}

pub fn todolist_list() -> Vec<String> {
    let mut file_vec: Vec<String> = Vec::new();
    let todolist_path = Path::new("./todolist");

    for file in fs::read_dir(todolist_path).unwrap(){
        let file = file.unwrap().file_name();
        file_vec.push(file.into_string().unwrap().replace(".json", ""));
    }
    file_vec

}

pub fn remove_todolist_file(name: &str)-> bool {
    let todolist_path = Path::new("./todolist");
    if todolist_path.is_dir(){
        let todolist_path = todolist_path.join(name);
        if todolist_path.exists(){
            fs::remove_file(todolist_path).expect("Unable to remove file");
            return true;
        }
    }
    false
}

pub fn add_to_todolist(name: &str, task: &str) {
    let todolist_path = Path::new("./todolist").join(format!("{}.json", name));
    println!("{}",todolist_path.to_str().unwrap());

    let file = File::open(todolist_path).expect("Unable to open file.");


    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");



}

pub fn read_todolist(name: &str) -> serde_json::Value{
    let todolist_path = Path::new("./todolist").join(format!("{}.json", name));

    let file = File::open(todolist_path).expect("Unable to open file.");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");

    // if let Some(obj) = json.as_object() {
    //     for (key, value) in obj {
    //         println!("{} -> {}", key, value.get("text").unwrap());
    //     }
    // }
    json

}













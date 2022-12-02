use rocket::serde::{json::Json, Deserialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Task<'r> {
    file_name: &'r str,
    description: String,
}

fn get_existing_content(file_name: &String) -> String {
    let file = File::open(&file_name);

    let mut buffer = String::new();

    match file {
        Ok(mut file) => {
            file.read_to_string(&mut buffer).unwrap();
        }
        Err(_) => {
            println!("File does not exist");
        }
    }

    buffer
}

#[post("/", format = "application/json", data = "<task>")]
pub fn write_file(task: Json<Task<'_>>) -> String {
    let mut file_name = task.file_name.to_owned();

    file_name.push_str(".txt");

    let mut file_content = String::new();

    let existing_content = get_existing_content(&file_name);

    if existing_content.len() > 0 {
        file_content.push_str(&existing_content)
    }

    file_content.push_str(&task.description);

    let mut file = File::create(file_name).unwrap();

    file.write_all(file_content.as_bytes()).unwrap();

    format!("File {} created", task.file_name)

    // TODO send to user the created file
}

use std::{
    io,
    io::{BufRead}
};

struct Todo {
    task: String,
    done: bool
}

impl Todo {
    fn create(task: String, done: bool) -> Todo {
        Todo {task, done,}
    }
}
fn main() {

    loop {
        
        let mut action = String::new();
        let mut task: String = String::new();
        let stdin = io::stdin();
        println!("What do you want to do?");
        action = stdin.lock().lines().next().unwrap().unwrap();

        if action == "break"{
            break;
        };

        print!("What is your task?");
        task = stdin.lock().lines().next().unwrap().unwrap();

        match action.as_ref() {
            "show" => print!("show task"),
            "create" => create_todo(),
            "complete" => print!("complete task"),
            "delete" => print!("delete task"),
            _ => print!("unknown action")
        }


    }
}

fn create_todo(){
    let new_todo = Todo::create("Read a book".to_string(), false);
    println!("Hello, world! {} - {} ", new_todo.task, new_todo.done);

}
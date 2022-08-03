use std::{
    io,
    io::{BufRead, Error, Read}, 
    fs::OpenOptions
};

#[derive(Debug)]
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

    let _todo: Vec<Todo> = all_todo().expect("error getting tasks");
    print!("Todo: {:?} ", _todo);

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
            "create" => create_todo(task, false),
            "complete" => print!("complete task"),
            "delete" => print!("delete task"),
            _ => print!("unknown action")
        }


    }
}

fn create_todo(task: String, done: bool){
    let new_todo = Todo::create("Read a book".to_string(), false);
    println!("Hello, world! {} - {} ", new_todo.task, new_todo.done);
}

fn all_todo() -> Result<Vec<Todo>, Error> {
    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .read(true)
    .open("todo.txt")
    .expect("Error!");

    let mut body = String::new();
    file.read_to_string(&mut body).expect("could not read the file");
    let mut list: Vec<Todo> = Vec::new();

    for line in body.lines() {
        let task = line.split(':').collect::<Vec<&str>>();
        list.push(Todo::create(task[0].to_string(), task[1].parse().unwrap()));
    }
    Ok(list)
}


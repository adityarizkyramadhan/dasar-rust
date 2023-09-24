use crate::todo::{Todo};
use std::io;

pub struct CommandLine {
    todos: Vec<Todo>,
}

impl CommandLine {
    pub fn new() -> CommandLine {
        CommandLine {
            todos: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let mut display = true;
        while display {
            println!("=======================================");
            println!("1. Insert todo");
            println!("2. Delete todo");
            println!("3. Print todo");
            println!("4. Mark as done todo");
            println!("5. Rename todo");
            println!("6. Exit");
            let mut input = String::new();
            println!("Input choice:");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input = input.trim().to_string();
            println!("Input: '{}'", input);
            println!("{}", input == "1");
            if input == "1"{
                self.insert()
            } else if input == "2"{
                self.delete()
            } else if input == "3"{
                self.print()
            }else if input == "6"{
                display = false;
            }else {
                println!("Wrong input")
            }
        }
    }

    fn insert(&mut self){
        let mut input = String::new();
        println!("Input data:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let todo = Todo::new(input);
        self.todos.push(todo);
    }

    fn delete(&mut self){
        let mut input = String::new();
        print!("Input ID:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        for (index, value) in self.todos.iter().enumerate(){
            if value.id == input {
                self.todos.remove(index);
                println!("Data with ID: {} is deleted", input);
                return;
            }
        }
        println!("No data is deleted")
    }

    fn print(&mut self) {
        for todo in self.todos.iter(){
             todo.print()
        }
    }
}

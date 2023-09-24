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
        let mut input = String::new();

        // Prompt the user for input
        println!("Enter something:");

        // Read a line of input from the user
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Print the user's input
        println!("You entered: {}", input);
        let mut todo = Todo::new(input);
        self.todos.push(todo);
    }

    pub fn print(&mut self) {
        for todo in self.todos.iter(){
             todo.print()
        }
    }
}

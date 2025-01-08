use std::io; // For input/output operations // For storing todos

//Define a struct for Todo Items
#[derive(Debug)]
struct TodoItem {
    id: u32,
    description: String,
    is_completed: bool,
}

//Define a struct for Todo List
struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    // Adds a new TodoItem to the list with the given description.
    // Automatically assigns a unique id.
    fn add_item(&mut self, description: String) {
        let new_id = self.generate_id();
        self.items.push(TodoItem {
            id: new_id,
            description,
            is_completed: false,
        });
        println!("Todo added successfully.");
    }

    fn complete_item(&mut self, id: u32) {
        // Iterate through the items in the list,
        // and mark the item with the given id as completed.
        for item in &mut self.items {
            if item.id == id {
                item.is_completed = true;
                println!("Todo marked as completed.");
                return;
            }
        }
        println!("Todo with ID {} not found.", id);
    }

    fn delete_item(&mut self, id: u32) {
        // If the item exists in the list,
        // remove it from the list.
        if let Some(index) = self.items.iter().position(|item| item.id == id) {
            self.items.remove(index);
            println!("Todo deleted successfully.");
        } else {
            println!("Todo with ID {} not found.", id);
        }
    }

    fn list_items(&self) {
        // If the list is empty, print a message.
        // Otherwise, print all the items in the list.
        if self.items.is_empty() {
            println!("No todos available.");
        } else {
            println!("Your Todo List:");
            for item in &self.items {
                println!(
                    "[{}] - {} - {}",
                    item.id,
                    item.description,
                    if item.is_completed {
                        "Completed"
                    } else {
                        "Pending"
                    }
                );
            }
        }
    }

    fn generate_id(&self) -> u32 {
        // If the list is empty, return 1 as the id.
        // Otherwise, return the id of the last item + 1.
        if self.items.is_empty() {
            1
        } else {
            self.items.last().unwrap().id + 1
        }
    }
}

fn main() {
    let mut todo_list = TodoList { items: Vec::new() };

    loop {
        println!("--- Todo CLI ---");
        println!("1. Add a Todo");
        println!("2. Complete a Todo");
        println!("3. Delete a Todo");
        println!("4. List Todos");
        println!("5. Exit");
        println!("Enter your choice:");

        let choice = read_input_u32();

        match choice {
            1 => {
                println!("Enter the description of the new todo:");
                let description = read_input_string();
                todo_list.add_item(description);
            }
            2 => {
                println!("Enter the ID of the todo to mark as completed:");
                let id = read_input_u32();
                todo_list.complete_item(id);
            }
            3 => {
                println!("Enter the ID of the todo to delete:");
                let id = read_input_u32();
                todo_list.delete_item(id);
            }
            4 => todo_list.list_items(),
            5 => {
                println!("Exiting Todo CLI. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

// Reads a line of input from the user and returns it as a string.
fn read_input_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

// Reads a line of input from the user and returns it as a u32.
fn read_input_u32() -> u32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    match input.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a number.");
            0
        }
    }
}

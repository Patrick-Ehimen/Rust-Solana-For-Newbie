use std::fs; // For file operations
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

fn main() {}

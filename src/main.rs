use std::io;

struct TodoList{
    title: String,
    description: String,
    checked: bool
}

fn main() {
    let mut 
    
    todo_lists: Vec<TodoList>  = vec![];

    println!("Initate todo list app");


    loop{
    
        println!("Enter title of your todo:");
        let mut title = String::new();
        io::stdin().read_line(&mut title).expect("Failed to read line");
        let title = title.trim().to_string();

        println!("Enter description of your todo:");
        let mut description = String::new();

        io::stdin().read_line(&mut description).expect("Failed to read line");
        let description = description.trim().to_string();


        todo_lists.push(TodoList {
            title,
            description,
            checked: false, 
        });


        println!("Todo added successfully!");


        println!("Do you want to add another todo? (yes/no)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        let answer = answer.trim().to_lowercase();

        if answer == "no" {
            break;
        };
        
    }

    println!("Final Todo List:");
    read_todo_list(todo_lists);

    
   
}


fn read_todo_list(todo_lists: Vec<TodoList>) {
    println!("Current Todo List:");

    if todo_lists.is_empty() {
        println!("No todos available.");
    } else {
        for (index, todo) in todo_lists.iter().enumerate() {
            let status = if todo.checked { "✔" } else { "✘" };
            println!("{}. [{}] {} - {}", index + 1, status, todo.title, todo.description);
        }
    }
}


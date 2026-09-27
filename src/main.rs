use colour::green_ln;
use std::io;

#[derive(Debug)]
struct Task {
    id: i32,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: i32, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
        }
    }
}

#[derive(Debug)]
struct TaskList {
    tasks: Vec<Task>
}

impl TaskList {

    fn new() -> Self {

        let tasks = Vec::new();

        Self { tasks }
    }

    fn find_next_id(&self) -> i32 {

        let mut new_id = 0;

        for task in &self.tasks {

            if task.id > new_id {
                new_id = task.id;
            }
        }
    
        new_id + 1
    }

    fn find_index(&self, index: i32) -> i32 {
        for (i, task) in self.tasks.iter().enumerate() {
            if task.id == index {
                return i as i32;
            }
        }
        -1
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn complete_task(&mut self, id: i32) -> i32 {
        let result = self.find_index(id);
        if result >= 0 {
            self.tasks[result as usize].completed = true;
        }
        result
    }

    fn rename_task(&mut self, id: i32){

        let result = self.find_index(id);
        if result == -1 {
            println!("Task not found");
        }
        else{
            println!("Task {} found, write new description", id);
            let mut new_name = String::new();
    
            io::stdin()
                .read_line(&mut new_name)
                .unwrap();
    
            self.tasks[result as usize].description = new_name.trim_end().to_string();
        }  
    }

    fn show_tasks(&self){
        
        for task in &self.tasks {
            if task.completed {
                green_ln!("[X] {} - {}",task.id, task.description);
            } else {
                println!("[ ] {} - {}", task.id, task.description);
            }
        }
    }
}


fn main() {
    let mut task_list = TaskList::new();

    let initial_tasks = ["Learn Rust structs",
                            "Install Rust",
                            "Push project to GitHub"];

    for task in initial_tasks {

        let next_id = task_list.find_next_id();
    
        task_list.add_task(
            Task::new(
                next_id,
                task.to_string()
            )
        );
    }
    task_list.complete_task(2);

    println!("Tasks ({}):", task_list.tasks.len());
    task_list.show_tasks();

    let mut running = true;

    while running {
        let mut instruction = String::new();

        io::stdin()
            .read_line(&mut instruction)
            .unwrap();

        let mut parts = instruction.trim().splitn(2, ' ');
        let command = parts.next().unwrap();
        let argument: String = parts.next().unwrap_or_default().to_string();

        if command == "add" {
            let new_task: Task = 
                Task::new(                            
                    task_list.find_next_id(),
                    argument
                );

            task_list.add_task(new_task);
        }

        else if command == "complete" {
            let index = argument.parse::<usize>().unwrap();            
            let result = task_list.complete_task(index as i32);
                        
            if result == -1 {
                println!("Task not found");
                continue;
            }
        }

        else if command == "print" {
            task_list.show_tasks();
        }

        else if command == "rename" {
            let index = argument.parse::<usize>().unwrap();       
            task_list.rename_task(index as i32);
        }

        else if command == "exit" {
            running = false;
        }
    }
}

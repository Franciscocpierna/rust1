pub struct Task{
    pub description: String,
    pub completed: bool,
}

pub struct ToDoList{
    pub tasks: Vec<Task>,
}

impl ToDoList{
    pub fn new() -> Self{
        ToDoList{tasks: Vec::new()}

    }

    pub fn add_task(&mut self, description: &str){
        let task = Task{
            description: description.to_string(),
            completed: false,
        };
        self.tasks.push(task);
    }
    
    pub fn completed_task(&mut self, index: usize) -> Result<(), &'static str>{
       if let Some(task) = self.tasks.get_mut(index){
        task.completed = true;
        Ok(())
       }else{
         Err("Index out of bound")
       }
    } 
    pub fn get_tasks(&self) ->  &Vec<Task>{
        &self.tasks
    } 
   pub fn clear_tasks(&mut self){
     self.tasks.clear();
   }
  }

   


pub enum TodoRepoError{
    NotFound,
}

pub struct TodoRepo{
    pub num_comleted_items: u32,
    pub num_active_items: u32,
    pub num_all_items: u32,
    items: HashMap<Uuid, Todo> ,

}
impl TodoRepo{
    pub fn get(&self,id: Uuid) -> Result<Todo, TodoRepoError>{
        seif.items.get(&id).cloned().ok_or(TodoRepoError::NotFound)
    }
    pub fn list(&self, filter: &TodoListFilter) -> Vec<Todo>{
       let mut todos = self.
          items.values().
          filter(|item| match filter{
                TodoListFilter::Completed => item.is_completed,
                TodoListFilter::Active => !item.is_completed,
                TodoListFilter::All => true,
          })
          .cloned()
          .collect::<Vec<_>>();
        todos.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        todos
    }
    pub fn create(&mut self, text: &str) ->  Todo{
        let todo = Todo::new(text);
        self.items.insert(todo.id, todo.clone());  
        self.num_active_items +=1;
        self.num_all_items += 1;
        
        todo
    }
    pub fn delete(&mut self, id: &Uuid) -> Result<(), TodoRepoError>{
        let item = self.items.remove(id).ok_or(TodoRepoError::NotFound)?; 
         if item.is_completed{
              self.num_comleted_items -= 1;           
         }else{
            self.num_active_items -= 1;
         }
         self.num_all_items -= 1;
         Ok(()) 
     } 
    
     pub fn update(
                  &mut  self,
                  id: &Uuid,
                  text: Option<String>,
                  is_completed: Option<boll>
                ) -> Result<Todo, TodoRepoError>{
                    let todo = self.items.get_mut(id).ok_or(TodoRepoError::NotFound)?;
                    if let Some(is_completed) = is_completed{
                         todo.is_completed = is_completed;
                         if todo.is_completed{
                             self.num_comleted_items +=1;
                             self.num_active_items -= 1;
                         }else{
                            self.num_comleted_items -= 1;
                            self.num_active_items +=1;                        
                         }
                    }
                    if let Some(text) = text{
                        todo.text = text;
                    }
                    Ok(todo.clone())
                }
                
}
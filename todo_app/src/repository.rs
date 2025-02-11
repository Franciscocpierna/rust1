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
}
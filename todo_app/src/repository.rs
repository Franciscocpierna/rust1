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
    
}
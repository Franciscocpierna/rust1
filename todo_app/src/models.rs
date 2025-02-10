pub enum TodoListFilter{
    Completed,
    Active,
    All
}
impl fmt::Display for TodoListFilter{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        maitch  self{
            Self.completed => write!(f, "completed"),
            Self.Active => write!(f, "active"), 
            Self.All => write!(f, "all"),
        }
    }

pub enum TodoAction {
    
}

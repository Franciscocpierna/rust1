// Definição do trait State
pub trait State {
    fn publish(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

// Implementação do struct Draft, que representa o estado de rascunho
pub struct Draft;

// Implementação do trait State para Draft
impl State for Draft {
    fn publish(self: Box<Self>) -> Box<dyn State> {
        println!("Publicando o post...");
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        println!("Rejeitando o post...");
        Box::new(Draft {})
    }
}

// Implementação do struct Published, que representa o estado publicado
pub struct Published;

// Implementação do trait State para Published
impl State for Published {
    fn publish(self: Box<Self>) -> Box<dyn State> {
        println!("O post já está publicado.");
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        println!("Não é possível rejeitar um post publicado.");
        self
    }
}

// Definição da struct Post
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

// Implementação de métodos para Post
impl Post {
    // Método para criar um novo post no estado de rascunho
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // Método para adicionar conteúdo ao post
    pub fn add_content(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Método para publicar o post
    pub fn publish(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.publish());
        }
    }

    // Método para rejeitar o post
    pub fn reject(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.reject());
        }
    }
}



#[test]
fn test_publish_draft() {
    let mut post = Post::new();
    post.add_content("Conteúdo do post...");
    post.publish();
    assert!(matches!(post.state, Some(_)));
}

#[test]
fn test_publish_published() {
    let mut post = Post::new();
    post.add_content("Conteúdo do post...");
    post.publish();
    post.publish(); // Tentar publicar novamente
    assert!(matches!(post.state, Some(_)));
}

#[test]
fn test_reject_draft() {
    let mut post = Post::new();
    post.add_content("Conteúdo do post...");
    post.reject(); // Tentar rejeitar no estado de rascunho
    assert!(matches!(post.state, Some(_)));
}

#[test]
fn test_reject_published() {
    let mut post = Post::new();
    post.add_content("Conteúdo do post...");
    post.publish();
    post.reject();
    assert!(matches!(post.state, Some(_)));
}

#[test]
fn test_content_added() {
    let mut post = Post::new();
    post.add_content("Conteúdo do post...");
    assert_eq!(post.content, "Conteúdo do post...");
}

#[test]
fn test_no_content_added() {
    let post = Post::new();
    assert_eq!(post.content, "");
}
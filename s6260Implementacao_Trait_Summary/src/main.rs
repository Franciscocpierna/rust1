// Definindo um trait Summary
trait Summary {
    fn summarize(&self) -> String;

    // Método associado para criar uma instância de notícia
    fn create_news(title: &str, content: &str) -> News {
        News { title: title.to_string(), content: content.to_string() }
    }
}

// Implementação do trait Summary para &str
impl Summary for &str {
    fn summarize(&self) -> String {
        format!("Breaking news! The message is: {}", self)
    }
}

// Implementação do trait Summary para String
impl Summary for String {
    fn summarize(&self) -> String {
        format!("Breaking news! The message is: {}", self)
    }
}

// Struct que representa uma notícia
struct News {
    title: String,
    content: String,
}

// Implementação do trait Summary para News
impl Summary for News {
    fn summarize(&self) -> String {
        format!("Breaking news! {}: {}", self.title, self.content)
    }
}

// Função notify que recebe um parâmetro que implementa o trait Summary
pub fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

fn main() {
    let message1 = "Hello, world!";
    let message2 = String::from("Rust is awesome!");

    notify(&message1);
    notify(&message2);

    let news = News::create_news("New Feature", "A new feature has been released!");
    notify(&news);
}

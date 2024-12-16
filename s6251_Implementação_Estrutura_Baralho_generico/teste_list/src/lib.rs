// Definição de uma estrutura genérica para representar uma carta
#[derive(Debug, PartialEq)]
pub struct Card<'a, Value> {
    value: &'a Value,
    suit: Suit,
}

// Enum para representar os naipes das cartas
#[derive(Debug, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

// Definição de uma estrutura para representar um baralho genérico
pub struct Deck<'a, Value> {
    cards: Vec<Card<'a, Value>>,
}

// Implementação do baralho
impl<'a, Value> Deck<'a, Value> {
    // Método para criar um novo baralho
    pub fn new() -> Deck<'a, Value> {
        Deck { cards: Vec::new() }
    }

    // Método para adicionar uma carta ao baralho
    pub fn add_card(&mut self, card: Card<'a, Value>) {
        self.cards.push(card);
    }

    
}

// Testes


#[test]
fn test_deck() {
    let mut deck: Deck<i32> = Deck::new();
    let card = Card { value: &10, suit: Suit::Hearts };
    deck.add_card(card);

    assert_eq!(deck.cards.len(), 1);
}



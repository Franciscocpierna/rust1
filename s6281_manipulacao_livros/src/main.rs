extern crate ordinal;

use std::collections::BTreeSet;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: u32,
}

impl Book {
    fn century(&self) -> u32 {
        (self.year - 1) / 100 + 1
    }
}

fn collection() -> Vec<Book> {
    vec![
        Book {
            title: String::from("Book 1"),
            author: String::from("Author 1"),
            year: 1875,
        },
        Book {
            title: String::from("Book 2"),
            author: String::from("Author 2"),
            year: 1950,
        },
        Book {
            title: String::from("Book 3"),
            author: String::from("Author 3"),
            year: 1802,
        },
        Book {
            title: String::from("Book 4"),
            author: String::from("Author 4"),
            year: 1987,
        },
        Book {
            title: String::from("Book 5"),
            author: String::from("Author 5"),
            year: 1901,
        },
    ]
}

fn main() {
    let books = collection();

    books.iter()
        .map(Book::century)
        .collect::<BTreeSet<_>>()
        .into_iter() // centuries in the ascending order
        .for_each(|century| {
            println!("Books from the {} century:", ordinal::Ordinal(century));
            books.iter()
                .enumerate()
                .filter_map(|(ix, b)| {
                    if b.century() == century {
                        Some((ix + 1, b)) // book's id is ix + 1
                    } else {
                        None
                    }
                })
                .for_each(|(id, book)| {
                    println!("#{}: {:?}", id, book);
                });
        });
}

struct Book {
    title: String,
    author: String,
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    // ここにメソッドを実装します
    fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }

    fn borrow_book(&self, title: &str) -> Option<&Book> {
        self.books.iter().find(|book| book.title == title)
    }

    // fn return_book(&self) {}

    fn display_books(&self) {
        for book in &self.books {
            println!("{}", book.title);
            println!("{}", book.author);
        }
    }
}

pub fn main() {
    // ここでLibraryを使用します
    let mut library = Library { books: vec![
        Book {
            title: String::from("book1"),
            author: String::from("for book1"),
        },
        Book {
            title: String::from("book2"),
            author: String::from("for book2"),
        },
        Book {
            title: String::from("book3"),
            author: String::from("for book3"),
        }
    ]};

    library.display_books();
    library.add_book(Book { title: "new".to_string(), author: "for new".to_string() });
    library.display_books();

    // Option<&Book> を適切に処理する
    if let Some(book) = library.borrow_book("new") {
        println!("title: {}, author: {}", book.title, book.author);
    } else {
        println!("Book not found");
    }

}
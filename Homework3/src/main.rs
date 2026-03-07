use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename).expect("failed to create file");  
    for book in books {
            writeln!(file, "{},{},{}", book.title, book.author, book.year).expect("failed to write!!");
        }

    }

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename).expect("could not open!");
    let reader = BufReader::new(file);
    
    let mut books: Vec<Book> = Vec::new();

        for line_result in reader.lines() {
            let line = line_result.expect("Can't read line!");

            let fields: Vec<&str> = line.split(',').collect();

    
            let title = fields[0].trim().to_string();
            let author = fields[1].trim().to_string();
            let year: u16 = fields[2].trim().parse().expect("invalid year");

            books.push(Book { title, author, year });
            

        }
    
    books

}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}
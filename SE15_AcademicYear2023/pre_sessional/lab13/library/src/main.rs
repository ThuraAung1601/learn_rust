mod library;
use library::{books::Book, media::AudioBook, LibraryItem};

fn main() {
    let mut book = Book::new(
        "The Rust Programming Language".to_string(),
        "Steve Klabnik and Carol Nichols".to_string(),
        "1593278284".to_string(),
    );

    let mut audiobook = AudioBook::new(
        "Rust in Action".to_string(),
        "Tim McNamara".to_string(),
        "John Doe".to_string(),
        480,
    );

    println!("Book title: {}", book.title());
    println!("AudioBook title: {}", audiobook.title());

    book.check_out();
    audiobook.check_out();

    println!("Is book available? {}", book.is_available());
    println!("Is audiobook available? {}", audiobook.is_available());

    book.check_in();
    audiobook.check_in();

    // Demonstrating use of trait objects
    let library_items: Vec<&dyn LibraryItem> = vec![&book, &audiobook];
    for item in library_items {
        println!("Library item: {}, Available: {}", item.title(), item.is_available());
    }
}
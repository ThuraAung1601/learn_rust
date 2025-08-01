pub struct Book {
    title: String,
    author: String,
    isbn: String,
    available: bool,
}

impl Book {
    pub fn new(title: String, author: String, isbn: String) -> Self {
        Book {
            title,
            author,
            isbn,
            available: true,
        }
    }
}

impl crate::library::LibraryItem for Book {
    fn title(&self) -> &str {
        &self.title
    }

    fn check_out(&mut self) {
        if self.available {
            self.available = false;
            println!("Book '{}' has been checked out.", self.title);
        } else {
            println!("Book '{}' is not available for checkout.", self.title);
        }
    }

    fn check_in(&mut self) {
        if !self.available {
            self.available = true;
            println!("Book '{}' has been checked in.", self.title);
        } else {
            println!("Book '{}' is already available.", self.title);
        }
    }

    fn is_available(&self) -> bool {
        self.available
    }
}

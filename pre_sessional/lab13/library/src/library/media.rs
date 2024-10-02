pub struct AudioBook {
    title: String,
    author: String,
    narrator: String,
    duration: u32, // in minutes
    available: bool,
}

impl AudioBook {
    pub fn new(title: String, author: String, narrator: String, duration: u32) -> Self {
        AudioBook {
            title,
            author,
            narrator,
            duration,
            available: true,
        }
    }
}

impl crate::library::LibraryItem for AudioBook {
    fn title(&self) -> &str {
        &self.title
    }

    fn check_out(&mut self) {
        if self.available {
            self.available = false;
            println!("AudioBook '{}' has been checked out.", self.title);
        } else {
            println!("AudioBook '{}' is not available for checkout.", self.title);
        }
    }

    fn check_in(&mut self) {
        if !self.available {
            self.available = true;
            println!("AudioBook '{}' has been checked in.", self.title);
        } else {
            println!("AudioBook '{}' is already available.", self.title);
        }
    }

    fn is_available(&self) -> bool {
        self.available
    }
}
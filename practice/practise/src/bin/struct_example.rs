// struct

#[derive(Debug)]
struct Book {
    author: String,
    price: usize
}

impl Book {
    fn new(name: String, price: usize) -> Book {
        Book {
            author: name,
            price: price
        }
    }
}

fn main() {
    // let b1 = Book {
    //     author: "John".to_string(),
    //     price: 32
    // };

    // let b2 = Book::new("Tom".to_string(), 34);

    let name = vec!["Tom", "John"]; // vec of &str
    let price_ls = vec![34, 32];
    let name_ls: Vec<String> = name.iter().map(|s| s.to_string()).collect();
    // println!("{:?}", name_ls);

    let mut book_ls = Vec::new();

    if name_ls.len() == price_ls.len() {
        for i in 0..name_ls.len() {
            // println!("{}, {}", name_ls[i], price_ls[i]);
            // **String**, **Vec** => clone()
            let b = Book::new(name_ls[i].clone(), price_ls[i]);
            book_ls.push(b);
        }
    }
    else {
        // eprintln!("Error");
        panic!("Not same length");
    }
    
    println!("{:?}", book_ls);

    // println!("{:?}", b1);
    // println!("{:?}", b2);
}
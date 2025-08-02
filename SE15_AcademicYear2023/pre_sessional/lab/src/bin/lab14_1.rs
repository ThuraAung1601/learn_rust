use std::thread;

fn main() {
    let mut handles = vec![];
    
    for i in 1..=5 {
        let handle = thread::spawn(move || {
            println!("Thread {} result: {}", i, i * i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

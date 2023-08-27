/*
// Rust "strings"
// 1. &str
// 2. String

let x: Vec<_> = std::env::args().collect();
if x.len() < 2 { return }

let greeting = format!("Hello, {}!", x[1]); // String

let second = format!("* {} *", " ".repeat(greeting.len())); // String

1: greeting.len(): (usize, n)
2: " ".repeat(n): (String, " ... n ... ")
3: format!("* {} *", " ... n ... "): (String, "* ... n ... *")
4: let second = "* ... n ... *"; => (String, "* ... n ... *")

{
    let mut i = b;
    while i < e {
        // ...
        i += 2
    }
}

// 0..3
for i in b..e {
    // ...
}
*/

fn main() {
    let x: Vec<_> = std::env::args().collect();
    if x.len() < 2 { return }

    let greeting = format!("Hello, {}!", x[1]);

    let pad = 4;
    let rows = pad * 2 + 3;
    let cols = greeting.len() + pad * 2 + 2;

    // invariant: we have written `r' rows so far
    for r in 0..rows {
        let mut c = 0;
        // invariant: we have written `c' characters
        // so far in the current row
        while c != cols {
            if r == pad + 1 && c == pad + 1 {
                print!("{}", greeting);
                c += greeting.len();
            }
            else {
                // are we on the border?
                if r == 0 || r == rows - 1
                   || c == 0 || c == cols - 1 {
                    print!("*");
                }
                else {
                    print!(" ");
                }
                c += 1;
            }
        }

        println!();
    }
}

fn main1() {
    let x: Vec<_> = std::env::args().collect();
    if x.len() < 2 { return }

    let greeting = format!("Hello, {}!", x[1]);

    let pad = 2;
    let rows = pad * 2 + 3;
    let cols = greeting.len() + pad * 2 + 2;

    let mut r = 0;
    // invariant: we have written `r' rows so far
    while r != rows {
        let mut c = 0;
        // invariant: we have written `c' characters
        // so far in the current row
        while c != cols {
            if r == pad + 1 && c == pad + 1 {
                print!("{}", greeting);
                c += greeting.len();
            }
            else {
                // are we on the border?
                if r == 0 || r == rows - 1
                   || c == 0 || c == cols - 1 {
                    print!("*");
                }
                else {
                    print!(" ");
                }
                c += 1;
            }
        }

        println!();
        r += 1;
    }
}

fn main0() {
    let x: Vec<_> = std::env::args().collect();
    if x.len() < 2 { return }

    let greeting = format!("Hello, {}!", x[1]);

    let second = format!("* {} *", " ".repeat(greeting.len()));
    let first = "*".repeat(second.len());

    println!("{}", first);
    println!("{}", second);
    println!("* {} *", greeting);
    println!("{}", second);
    println!("{}", first);
}

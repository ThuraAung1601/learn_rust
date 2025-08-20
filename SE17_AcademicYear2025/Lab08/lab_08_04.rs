use std::mem;

// 1. Setting Up the Structure
// An enum to represent a node in the linked list.
// `Box<Node>` allows for a recursive type because Box is a pointer
// with a known size, breaking the infinite size calculation.
#[derive(Debug)] // <-- ADDED THIS LINE TO FIX THE ERROR
enum Node {
    // A node containing a value (i32) and a pointer to the next node.
    Cons(i32, Box<Node>),
    // The end of the list.
    Nil,
}

// The stack struct itself.
// Using `derive(Debug)` allows us to print the struct for debugging.
#[derive(Debug)]
pub struct BoxedStack {
    // `top` always points to the head of the linked list.
    top: Box<Node>,
}

// 2. Adding Functions (Implementation)
impl BoxedStack {
    /// Creates a new, empty stack.
    pub fn new() -> Self {
        BoxedStack {
            top: Box::new(Node::Nil),
        }
    }

    /// Pushes a new value onto the top of the stack.
    pub fn push(&mut self, value: i32) {
        // `mem::replace` swaps the value of `self.top` with a new `Nil` node,
        // giving us ownership of the old `top` node.
        let old_top = mem::replace(&mut self.top, Box::new(Node::Nil));
        
        // Create a new `Cons` node with the given value, pointing to the old top.
        let new_top = Box::new(Node::Cons(value, old_top));
        
        // Set the stack's top to our newly created node.
        self.top = new_top;
    }

    /// Pops the top value from the stack, if it exists.
    pub fn pop(&mut self) -> Option<i32> {
        // Replace the current top with a Nil node, taking ownership of the old top.
        let old_top = mem::replace(&mut self.top, Box::new(Node::Nil));

        // Match on the node we just took ownership of.
        match *old_top {
            // If it was a `Cons` node, we have a value.
            Node::Cons(value, next_node) => {
                // The stack's top becomes the next node in the list.
                self.top = next_node;
                // Return the value.
                Some(value)
            }
            // If it was `Nil`, the stack was empty.
            Node::Nil => None,
        }
    }

    /// Returns a reference to the top value without removing it.
    pub fn peek(&self) -> Option<&i32> {
        // Match on a reference to the node pointed to by `self.top`.
        match &*self.top {
            Node::Cons(value, _) => Some(value),
            Node::Nil => None,
        }
    }

    /// Checks if the stack is empty.
    pub fn is_empty(&self) -> bool {
        // The stack is empty if the top is a `Nil` node.
        matches!(&*self.top, Node::Nil)
    }

    /// Prints the contents of the stack from top to bottom.
    pub fn print_stack(&self) {
        print!("Stack contents: ");
        // Start the recursive printing process from the top node.
        Self::print_recursive_helper(&self.top);
        println!();
    }

    // A private, recursive helper function for printing.
    fn print_recursive_helper(node: &Node) {
        match node {
            Node::Cons(value, next_node) => {
                print!("{} -> ", value);
                Self::print_recursive_helper(next_node);
            }
            Node::Nil => {
                print!("Nil");
            }
        }
    }
}

fn main() {
    let mut stack = BoxedStack::new();

    println!("Pushing 10 onto the stack.");
    stack.push(10);
    println!("Pushing 20 onto the stack.");
    stack.push(20);
    println!("Pushing 30 onto the stack.");
    stack.push(30);

    stack.print_stack();

    if let Some(top_val) = stack.peek() {
        println!("Top of the stack: {}", top_val);
    }

    if let Some(popped_val) = stack.pop() {
        println!("Popped {} from the stack.", popped_val);
    }
    stack.print_stack();

    if let Some(popped_val) = stack.pop() {
        println!("Popped {} from the stack.", popped_val);
    }
    stack.print_stack();
    
    if let Some(popped_val) = stack.pop() {
        println!("Popped {} from the stack.", popped_val);
    }
    stack.print_stack();

    println!("Is the stack empty? {}", stack.is_empty());
}

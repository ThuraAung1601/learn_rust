fn main() {
    let mut v: Vec<i32> = Vec::new();

    // address of the Vec header (on the stack)
    println!("Vec header (stack) = {:p}", &v as *const _);

    for i in 0..80 {
        v.push(i);
        // address of the heap buffer (may change when capacity grows)
        println!(
            "len: {:>2}, cap: {:>2}, buf: {:p}",
            v.len(),
            v.capacity(),
            v.as_ptr()
        );
    }
}

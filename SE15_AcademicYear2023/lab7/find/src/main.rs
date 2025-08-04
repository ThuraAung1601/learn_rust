fn main() {
    if let Err(e) = find::get_args().and_then(find::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
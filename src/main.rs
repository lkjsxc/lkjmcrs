fn main() {
    if let Err(error) = lkjmcrs::app::main() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

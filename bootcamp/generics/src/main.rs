#[derive(Debug)]
struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand { name, payload }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}: {}", self.name, self.payload);
    }
}
fn main() {
    let cmd1 = BrowserCommand::new("navigate".to_owned(), "https://engramsoft.com".to_owned());

    let cmd2 = BrowserCommand::new("zoom".to_owned(), 200);

    println!("{cmd1:?}");
    println!("{cmd2:?}");

    cmd1.print_payload();
    let payload = cmd2.get_payload();
    println!("{payload}");
}

// When using generics in rust there is no runtime cost
// generics is just as fast as to create two separated functions
// that is because this is exactly what Rust does at compile time
// through a process called Monomorphization.

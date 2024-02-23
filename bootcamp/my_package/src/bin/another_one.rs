// packages could have at most one library crate,
// // but any amount of binary crates

// each file created in the src/bin directory created as another binary crate
// cargo run does not work anymore if you have more than one binary crate
// // you need to specify the binary name with the "--bin" <binary_name>
fn main() {
    println!("Hello from another one");
}

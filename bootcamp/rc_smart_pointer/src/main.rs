use std::{clone, rc::Rc};
struct Database {}

struct AuthService {
    db: Rc<Database>,
}

struct ContentService {
    db: Rc<Database>,
}
fn main() {
    // the Rc smart pointer is similar to shared_pointer in C++
    let db = Rc::new(Database {});
    // the clone function in the Rc module is not actually cloning the the database
    // it will simply increment the reference counter of the smart pointer
    let auth_service = AuthService { db: Rc::clone(&db) };
    println!("{}", Rc::strong_count(&db));
    let content_service = ContentService { db: Rc::clone(&db) };
    println!("{}", Rc::strong_count(&db));
}

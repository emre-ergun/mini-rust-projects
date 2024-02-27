use std::rc::Rc;
use std::cell::RefCell;

struct Database {
    max_connections: u32,
}

struct AuthService {
    db: Rc<RefCell<Database>>,
}

struct ContentService {
    db: Rc<RefCell<Database>>,
}
fn main() {
    // the Rc smart pointer is similar to shared_pointer in C++
    let db = Rc::new(RefCell::new(Database {
        max_connections: 100,
    }));
    // the clone function in the Rc module is not actually cloning the the database
    // it will simply increment the reference counter of the smart pointer
    let auth_service = AuthService { db: Rc::clone(&db) };
    println!("{}", Rc::strong_count(&db));
    let content_service = ContentService { db: Rc::clone(&db) };
    println!("{}", Rc::strong_count(&db));
    //Rc smart pointer only allows immutable shared ownership of a value.
    //db.max_connections = 200;
    // after RefCell, it works
    //db.borrow_mut().max_connections = 200;
    let mut r1 = db.borrow_mut();
    let r2 = db.borrow_mut();
    r1.max_connections = 200;

}

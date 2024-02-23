use auth_service::Credentials;

fn main() {
    let creds = Credentials::new(String::from("Emre"), String::from("123456"));
    println!("{creds:?}");

    auth_service::authenticate(creds);
}
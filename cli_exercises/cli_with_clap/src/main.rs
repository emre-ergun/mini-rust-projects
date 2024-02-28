use clap::{command, Arg};
fn main() {
    let match_result = command!()
    .arg(
        Arg::new("firstname").short('f').long("fname")
    )
    .arg(
        Arg::new("lastname").short('l').long("lname")
    )
    .version("v0.1.0")
    .get_matches();

}

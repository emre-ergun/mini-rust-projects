use clap::{command, Arg, ArgGroup, Command};
fn main() {
    let match_result = command!()
        .subcommand(
            Command::new("register-person")
                .arg(
                    Arg::new("firstname")
                        .short('f')
                        .long("first-name")
                        .aliases(["fname", "firstname"])
                        .required(true)
                        .help("Take the person's first name"),
                )
                .arg(
                    Arg::new("lastname")
                        .short('l')
                        .long("last-name")
                        .aliases(["lname", "lastname"])
                        .required(true)
                        .help("Take the person's last name"), //.conflicts_with("firstname"),
                ),
        )
        .subcommand(
            Command::new("register-pet").arg(
                Arg::new("petname")
                    .short('p')
                    .long("petname")
                    .aliases(["pet-name", "pname"])
                    .required(true)
                    .help("Take the pet's name"),
            ),
        )
        .about("This application registers people with their doctor's office")
        .arg(
            Arg::new("fluffy")
                .long("fluffy")
                .help("Specifiy if the person wears a fluffy coat 🥼 or not"),
        )
        .version("v0.1.0")
        .get_matches();
}

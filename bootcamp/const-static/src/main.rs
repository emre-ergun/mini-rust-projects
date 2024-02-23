// constant variables can not be mutated.
// naming convention of constant variables is screaming snake case
// constant variables can be decleared in any scope including global scope
// value has to be constant expression which means the value has to able to calculated at compile time
const MAX_PLAYERS: u8 = 8;

// naming convention of static variables is screaming snake case
// static variables can be decleared in any scope including global scope
// static variables, unlike constant varaibles, can be marked as mutable by "mut" keyword.
// // however, accessing or modifiying a mutable static variable is unsafe. It must be done within an unsafe block.
static CASINO_NAME: &str = "RustCasino";

fn main() {
    // for a and be MAX_PLAYERS will be inlined by the value assigned.
    // It means constants do not occupy a specific location in memory
    let a = MAX_PLAYERS;
    let b = MAX_PLAYERS;

    // on the other hand, statics do occupy a specific location in memory
    // // which means there is only one instance of the value
    let c = CASINO_NAME;
    let d = CASINO_NAME;

    // default to use constant variables instead of static variables.
    println!("{a}-{c}-{b}-{d}");
}

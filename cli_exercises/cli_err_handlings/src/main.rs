use anyhow::{Context, Result};

#[derive(Debug)]
struct CustomError(String);

// fn main() -> Result<(), CustomError> {
//     let path = "test.txt";
//     let content = std::fs::read_to_string(&path)
//         .map_err(|err| {
//             CustomError(format!("Error reading {path}: {err}"))
//         })?;
//     println!("{content}");

//     Ok(())
// }

fn main() -> Result<()> {
    let path = "test.txt";
    let content = std::fs::read_to_string(&path)
        .with_context(|| {
            format!("Coult not read the file '{path}'")
        })?;
    println!("Content: {content}");

    Ok(())
}

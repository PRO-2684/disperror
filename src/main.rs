use disperror::DispError;
use std::io::Read;

fn main() -> Result<(), DispError<std::io::Error>> {
    let mut file = std::fs::File::open("nonexistent_file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}

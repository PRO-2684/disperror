use disperror::DispError;

fn main() -> Result<(), DispError<std::io::Error>> {
    let contents = std::fs::read_to_string("nonexistent_file.txt")?;
    println!("{}", contents);
    Ok(())
}

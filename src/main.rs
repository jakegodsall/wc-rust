use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() -> Result<(), std::io::Error> {
    let content = read_file("data/test.txt")?;
    println!("{}", content);
    Ok(())
}
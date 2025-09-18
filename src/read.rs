use std::io::{ Read, BufReader };

pub fn read_stdin() -> Result<String, std::io::Error> {
  let mut content = String::new();
  std::io::stdin()
    .read_line(&mut content)?;
  Ok(content)
}

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let file: std::fs::File = std::fs::File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}
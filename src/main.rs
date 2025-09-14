use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

enum Mode { All, Bytes, Words, Lines, Chars }

fn main() -> Result<(), std::io::Error> {
  let args: Vec<String> = std::env::args().collect();

  let mut mode = Mode::All;
  let mut file_name: Option<String> = None;

  let mut i = 1;
  while i < args.len() {
    match args[i].as_str() {
        "-c" => mode = Mode::Bytes,
        "-w" => mode = Mode::Words,
        "-l" => mode = Mode::Lines,
        "-m" => mode = Mode::Chars,
        str => file_name = Some(str.to_string())
    }
  }

  println!("{}", file_name);
  Ok(())
}
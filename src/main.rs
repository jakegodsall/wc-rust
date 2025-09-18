use std::io::{ Read, BufReader };
use wc_rust::counts::{ count_words, count_bytes, count_lines, count_chars, output_all };

fn read_stdin() -> Result<String, std::io::Error> {
  let mut content = String::new();
  std::io::stdin()
    .read_line(&mut content)?;
  Ok(content)
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let file: std::fs::File = std::fs::File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

enum RunMode { All, Bytes, Words, Lines, Chars }

fn main() -> Result<(), std::io::Error> {

  let mut run_mode: RunMode = RunMode::All;
  let mut file_name: Option<String> = None;
  
  for arg in std::env::args().skip(1) { // skip command name
    match arg.as_str() {
      "-c" => run_mode = RunMode::Bytes,
      "-w" => run_mode = RunMode::Words,
      "-l" => run_mode = RunMode::Lines,
      "-m" => run_mode = RunMode::Chars,
      _ if arg.starts_with("-") => {
        println!("Unknown option: {}", arg);
        std::process::exit(1);
      }
      _ => {
        file_name = Some(arg);
      }
    }
  }

  let content = match file_name.as_deref() {
    Some(path) => read_file(path)?,
    None => read_stdin()?,
  };

  let value: String;
  match run_mode {
    RunMode::All => value = output_all(&content),
    RunMode::Bytes => value = count_bytes(&content).to_string(),
    RunMode::Words => value = count_words(&content).to_string(),
    RunMode::Lines => value = count_lines(&content).to_string(),
    RunMode::Chars => value = count_chars(&content).to_string(),
  }

  println!("{} {}", value, file_name.unwrap());
  Ok(())
}
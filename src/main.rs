use std::io::Read;
use wc_rust::counts::{ count_words, count_bytes, count_lines, count_chars };

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

enum RunMode { All, Bytes, Words, Lines, Chars }

fn main() -> Result<(), std::io::Error> {
  let args: Vec<String> = std::env::args().collect();

  let mut run_mode: RunMode = RunMode::All;
  let mut file_name: Option<String> = None;
  let content = read_file(file_name.as_deref().unwrap())?;

  let mut i: usize = 1;
  while i < args.len() {
    match args[i].as_str() {
        "-c" => run_mode = RunMode::Bytes,
        "-w" => run_mode = RunMode::Words,
        "-l" => run_mode = RunMode::Lines,
        "-m" => run_mode = RunMode::Chars,
        str => file_name = Some(str.to_string())
    }
    i += 1;
  }

  let count: usize;
  match run_mode {
    RunMode::Bytes => count = count_bytes(&content),
    RunMode::Words => count = count_words(&content),
    RunMode::Lines => count = count_lines(&content),
    RunMode::Chars => count = count_chars(&content),
    _ =>  count = 0,
  }

  println!("{} {}", count, file_name.unwrap());
  Ok(())
}
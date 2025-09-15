use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

enum RunMode { All, Bytes, Words, Lines, Chars }

fn count_bytes() -> usize {
  todo!();
}

fn count_words() -> usize {
  todo!();
}

fn count_lines() -> usize {
  todo!();
}

fn count_chars() -> usize {
  todo!();
}

fn main() -> Result<(), std::io::Error> {
  let args: Vec<String> = std::env::args().collect();

  let mut run_mode: RunMode = RunMode::All;
  let mut file_name: Option<String> = None;

  let mut i: usize = 1;
  while i < args.len() {
    match args[i].as_str() {
        "-c" => run_mode = RunMode::Bytes,
        "-w" => run_mode = RunMode::Words,
        "-l" => run_mode = RunMode::Lines,
        "-m" => run_mode = RunMode::Chars,
        str => file_name = Some(str.to_string())
    }
  }

  let mut count: usize;
  match run_mode {
    RunMode::Bytes => count = count_bytes(),
    RunMode::Words => count = count_words(),
    RunMode::Lines => count = count_lines(),
    RunMode::Chars => count = count_chars(),
    _ =>  count = 0,
  }

  println!("{} {}", count, file_name.unwrap());
  Ok(())
}
use std::io::{ Read, BufReader };
use wc_rust::counts::{ count_words, count_bytes, count_lines, count_chars, output_all };

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let file: std::fs::File = std::fs::File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

enum RunMode { All, Bytes, Words, Lines, Chars }

fn main() -> Result<(), std::io::Error> {
  let args: Vec<String> = std::env::args().collect();

  let mut run_mode: RunMode = RunMode::All;
  let mut file_name: Option<String> = None;
  
  let mut i: usize = 1;

  if args.len() > 2 {
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
  } else {
    match args[1].as_str() {
      str => {
        run_mode = RunMode::All;
        file_name = Some(str.to_string());
      }
    }
  }
  

  let content = read_file(file_name.as_deref().unwrap())?;

  let value: String;
  match run_mode {
    RunMode::All => value = output_all(&content),
    RunMode::Bytes => value = count_bytes(&content).to_string(),
    RunMode::Words => value = count_words(&content).to_string(),
    RunMode::Lines => value = count_lines(&content).to_string(),
    RunMode::Chars => value = count_chars(&content).to_string(),
    _ =>  value = String::from("0"),
  }

  println!("{} {}", value, file_name.unwrap());
  Ok(())
}
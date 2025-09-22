use wc_rust::counts::{ count_words, count_bytes, count_lines, count_chars, output_all };
use wc_rust::read::{ read_file, read_stdin };

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

  let value: String = match run_mode {
    RunMode::All => output_all(&content),
    RunMode::Bytes => count_bytes(&content).to_string(),
    RunMode::Words => count_words(&content).to_string(),
    RunMode::Lines => count_lines(&content).to_string(),
    RunMode::Chars => count_chars(&content).to_string(),
  }

  println!("{} {}", value, file_name.unwrap());
  Ok(())
}
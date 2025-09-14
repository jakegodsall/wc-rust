use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn print_output(value: usize, file_name: &str) {
    println!("{} {}", value, file_name)
}

fn count_bytes(data: &str) -> usize {
    data.len()
}

fn count_words(data: &str) -> usize {
    1
}

fn count_lines(data: &str) -> usize {
    1
}

fn count_chars(data: &str) -> usize {
    1
}


fn main() -> Result<(), std::io::Error> {
  let args: Vec<String> = std::env::args().collect();

  let mut i = 1;
  while i < args.len() {
    match args[i].as_str() {
        "-c" => {
            count_bytes("data/test.txt")
        },
        "-w" => {
            count_words("data/test.txt")
        },
        "-l" => {
            count_lines("data/test.txt")
        },
        "-m" => {
            count_chars("data/test.txt")
        }
        }
    }
    Ok(())
}
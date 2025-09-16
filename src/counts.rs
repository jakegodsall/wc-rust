/// Count the number of words in a string slice.
/// 
/// # Example
/// 
/// ```
/// let content = "hello, world";
/// let word_count = wc_rust::counts::count_words(content);
/// 
/// assert_eq!(2, word_count);
/// ```
pub fn count_words(content: &str) -> usize {
  content.split_whitespace().count()
}

/// Count the number of bytes in a string slice.
/// 
/// # Example
/// 
/// ```
/// let content = "hello, world";
/// let byte_count = wc_rust::counts::count_bytes(content);
/// 
/// assert_eq!(12, byte_count);
/// ```
pub fn count_bytes(content: &str) -> usize {
  content.as_bytes().len()
}


/// Count the number of lines in the string slice.
/// 
/// # Example
/// 
/// ```
/// let content = "hello\nworld\n";
/// let line_count = wc_rust::counts::count_lines(content);
/// 
/// assert_eq!(2, line_count);
/// ```
pub fn count_lines(content: &str) -> usize {
  content.lines().count()
}

/// Count the number of chars in the string slice.
/// 
/// # Example
/// 
/// ```
/// let content = "hello, world";
/// let char_count = wc_rust::counts::count_chars(content);
/// 
/// assert_eq!(12, char_count);
/// ```
pub fn count_chars(content: &str) -> usize {
  content.chars().collect::<Vec<_>>().len()
}

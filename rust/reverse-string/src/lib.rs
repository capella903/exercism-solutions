use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    UnicodeSegmentation::graphemes(input, true)
        .map(|c| String::from(c))
        .fold(String::new(), |acc, s| s + &acc)
}

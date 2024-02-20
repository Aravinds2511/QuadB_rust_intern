//3.Given a string of words, implement a function that returns the shortest word in the string
fn shortest_word(s: &str) -> Option<&str> {
    //splinting the str and finding the smallest word in len.
    s.split_whitespace()
     .min_by_key(|w| w.len())
}

fn main() {
     let sentence = "the shortest word in the sentence";
     match shortest_word(sentence) {
        Some(w) => println!("The shortest word is: '{}'", w),
        None => println!("No words found in the string"),
    }
}

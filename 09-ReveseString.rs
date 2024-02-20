//9. Reverse a string in Rust
fn reverse_string(s: &str) -> String {
    //using inbuild functions to reverse the sting
    return s.chars().rev().collect();
}

fn main() {
    let string = "reverse";
    println!("{}", reverse_string(string));
}

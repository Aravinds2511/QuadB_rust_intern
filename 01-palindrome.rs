//1. Implement a function that checks whether a given string is a palindrome or not.

fn is_palindrome(s: &str) -> bool {
    //get only alpha and nums and convert them to lowercase
    let clearStr: String = s.chars()
                             .filter(|c| c.is_alphanumeric())
                             .map(|c| c.to_ascii_lowercase())
                             .collect();
    //reverse and check for palindrome
    clearStr == clearStr.chars().rev().collect::<String>()
}    

fn main() {
    let test_str = "Racecar";
    let test2_str = "Testing";
    
    println!("{}",is_palindrome(test_str));
    println!("{}",is_palindrome(test2_str));
}

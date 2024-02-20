//6. Implement a function that finds the longest common prefix of a given set of strings
fn longest_common_prefix(strs: Vec<&str>) -> String {
    //when no strings
    if strs.is_empty() {
        return "".to_string();
    }

    let mut prefix = String::new();
    //getting the string with shortest length
    let shortest_str = strs.iter().min_by_key(|s| s.len()).unwrap();
    
    //iterating over the array of strings to find the longest common prefix
    for (i, ch) in shortest_str.chars().enumerate() {
        if strs.iter().all(|s| s.chars().nth(i) == Some(ch)) {
            prefix.push(ch);
        } else {
            break;
        }
    }

    return prefix;
}

fn main() {
    let strs = vec!["flower", "flow", "flight"];
    println!("Longest common prefix: {}", longest_common_prefix(strs));
}

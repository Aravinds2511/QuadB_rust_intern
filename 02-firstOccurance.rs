//2. Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number
fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut count = 0;
    // iterating and count inc for indentifying the index
    for &i in arr{
        if i == target{
            return Some(count);
        }
        else{
            count+=1;
        }
    }
    None
}

fn main() {
    let arr = [1, 2, 4, 4, 5, 6, 6];
    let arr2 = [2,33,45,45,55,66,77,77,77];
    let target = 6;
    match find_first_occurrence(&arr, target) {
        Some(count) => println!("First occurrence of {} is at index {}", target, count),
        None => println!("{} not found in the array", target),
    }
}

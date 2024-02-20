//7. Implement a function that returns the kth smallest element in a given array
fn kth_smallest_elem(arr: &mut [i32], k: usize) -> Option<i32> {
    //checking end conditions
    if k == 0 || k > arr.len() {
        return None;
    }
    //sorting the array
    arr.sort();
    //returning the element at the asked position
    Some(arr[k - 1])
}

fn main() {
    let mut arr = vec![2, 34, 5, 76, 12, 21, 7];
    let k = 3;
    match kth_smallest_elem(&mut arr, k) {
        Some(val) => println!("The smallest element in rank {} is: {}", k, val),
        None => println!("Invalid input"),
    }
}

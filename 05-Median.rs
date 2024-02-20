//5. Given a sorted array of integers, implement a function that returns the median of the array
fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    //for no elements in the arr
    if len == 0 {
        return f64::NAN;
    }
    //if the lenght is odd
    if len % 2 != 0 {
        arr[len / 2] as f64
    } else { //if the length is even
        (arr[len / 2] as f64 + arr[len / 2 - 1] as f64) / 2.0
    }
}

fn main() {
    let nums = [1, 2, 3, 5, 6];
    let numss = [1, 2, 3, 4, 5, 6];
    println!("The median is {}", median(&nums));
     println!("The median is {}", median(&numss));
}


//12. Find the maximum subarray sum in Rust
fn max_subarray_sum(arr: &[i32]) -> i32 {
    //return variable
    let mut max_so_far = i32::MIN;
    //comparing variable 
    let mut max_ending_here = 0;
    
    //using Kadane's Algorithm.....
    //iterating through the array
    for &val in arr {
        max_ending_here += val;
        //compares both the variables
        if max_so_far < max_ending_here {
            max_so_far = max_ending_here;
        }
        //zero less comparing variable start from 0.
        if max_ending_here < 0 {
            max_ending_here = 0;
        }
    }

   return max_so_far;
}

fn main() {
    let arr = [-2, -3, 4, -1, -2, 1, 5, 2, -2, 2, -3];
    println!("Maximum contiguous sum is {}", max_subarray_sum(&arr));
}

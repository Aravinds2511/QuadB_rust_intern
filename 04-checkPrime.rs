//4. Implement a function that checks whether a given number is prime or not
fn is_prime(n: u64) -> bool {
    //less the eq to 1 retun false
    if n <= 1 {
        return false;
    }
    //check for divisibility for the num from 2 to the half of the num
    for i in 2..=(n / 2) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let number = 29;
    println!("{}" ,is_prime(number));
}

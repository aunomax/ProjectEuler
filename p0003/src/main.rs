fn main() {
    let number = 600851475143;
    let mut lpf = 2;
    let mut n = 2;
    while n * n <= number {
        if number % n == 0 && is_prime(n) {
            lpf = n;
        }
        n += 1;
    }

    println!("The largest prime factor of {number} is {lpf}.");
}

fn is_prime(number: u64) -> bool {
    let mut i = 2;
    while i * i <= number {
        if number % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

/// # Largest Palindrome Product
/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91*99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    let mut max = 999 * 999;
    let min = 100 * 100;
    let mut flag = true;
    while flag && max > min {
        if is_symmetric(max) && is_mod(max) {
            flag = false;
            println!("{}", max);
        }
        max -= 1;
    }
}

fn is_mod(n: u64) -> bool {
    for r in 100..1000 {
        if n % r == 0 && n / r > 99 && n / r < 1000 {
            return true;
        }
    }
    return false;
}

fn is_symmetric(n: u64) -> bool {
    return n == inverse(n);
}

fn inverse(n: u64) -> u64 {
    let mut tmp = n;
    let mut reverse = 0;
    while tmp != 0 {
        reverse = tmp % 10 + reverse * 10;
        tmp /= 10;
    }
    return reverse;
}

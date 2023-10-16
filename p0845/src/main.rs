/// # Prime Digit Sum
///
/// Let D(n) be the n-th positive integer that has the sum of its digits a prime.
/// For example, D(61) = 157 and D(10^8) = 403539364.
///
/// Find D(10^16).

fn main() {
    println!("Hello, world!");
    //println!("{}", prime_digit_sum(10_usize.pow(9)));
    assert_eq!('a' < 'b', true);
    println!("{:?}", primes(100));
    println!("{:?}", count_primes(100, &primes(110)));
    assert_eq!(count_integers_nth(5), 11);
    assert_eq!(count_integers_nth(10_usize.pow(8)), 403539364);
}

// get the sum of positive integers that has the sum of its digits a prime and below 10*n.
pub fn count_integers_nth(n: usize) -> usize {
    let mut count = 0;
    let mut num: usize = 0;
    let primes = primes(200);
    let v = count_primes(188, &primes);
    while count <= n {
        count += v[digit_sum(num)];
        num += 10;
    }
    while count > n {
        num -= 1;
        if primes.contains(&(digit_sum(num))) {
            count -= 1;
        }
    }
    return num;
}

// get a vector that counts the number of primes between n and n + 10
pub fn count_primes(n: usize, v: &Vec<usize>) -> Vec<usize> {
    let mut count = Vec::new();
    for i in 0..n + 1 {
        let mut tmp = 0;
        for j in 0..10 {
            if v.contains(&(i + j)) {
                tmp += 1;
            }
        }
        count.push(tmp);
    }
    return count;
}

// get a vector that contains primes below n.
pub fn primes(n: usize) -> Vec<usize> {
    let mut tmp = Vec::new();
    for i in 1..n + 1 {
        if is_prime(i) {
            tmp.push(i);
        }
    }
    return tmp;
}

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

pub fn digit_sum(n: usize) -> usize {
    let mut sum = 0;
    let mut tmp = n;
    while tmp != 0 {
        sum += tmp % 10;
        tmp /= 10;
    }
    return sum;
}

// return n-th positive integer that has the sum of its digits a prime
pub fn prime_digit_sum(n: usize) -> usize {
    let mut count = 0;
    let mut num = 1;
    while count < n {
        num += 1;
        if is_prime(digit_sum(num)) {
            count += 1;
        }
    }
    return num;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_prime_digit_sum() {
        assert_eq!(prime_digit_sum(61), 157);
        //assert_eq!(prime_digit_sum(10_usize.pow(8)), 403539364);
        //assert_eq!(prime_digit_sum(10_usize.pow(9)), 4112370371);
        //assert_eq!(prime_digit_sum(10_usize.pow(10)), 42602833010);
        assert_eq!(prime_digit_sum(10_usize.pow(8)), 403539364);
    }
}

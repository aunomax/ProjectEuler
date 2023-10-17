use std::collections::HashMap;
/// # Prime Digit Sum
///
/// Let D(n) be the n-th positive integer that has the sum of its digits a prime.
/// For example, D(61) = 157 and D(10^8) = 403539364.
///
/// Find D(10^16).
use std::time::Instant;

fn main() {
    let now = Instant::now();
    println!("Hello, world!");
    //println!("{}", prime_digit_sum(10_usize.pow(9)));
    //println!("{:?}", primes(100));
    //println!("{:?}", count_primes(100, &primes(110)));
    //assert_eq!(count_integers_nth(5), 11);
    //assert_eq!(count_integers_nth(10_usize.pow(8)), 403539364);
    //assert_eq!(count_integers_nth(10_usize.pow(9)), 4112370371);
    //assert_eq!(count_integers_nth(10_usize.pow(10)), 42602833010);
    //assert_eq!(count_integers_nth(10_usize.pow(11)), 438589762252);

    //println!("{}", count_integers_nth(10_usize.pow(16)));

    println!("{:?}", now.elapsed());
    let primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199,
    ];
    let vv = [
        4, 4, 5, 4, 4, 4, 3, 3, 3, 3, 4, 4, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 3, 3, 2, 2, 2, 2, 3, 3,
        2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 3, 3, 2, 2, 2, 2, 3, 3,
        2, 2, 2, 2, 3, 3, 3, 3, 2, 2, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2,
        1, 1, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 3, 3, 3, 3, 3, 3, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1,
        1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3,
        2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 3, 3, 2, 2, 2, 2, 2, 2,
        1, 1, 1, 1, 2, 2, 2, 2, 3,
    ];
    let mut count = 0;
    let mut num: usize = 0;
    const N: usize = 10_usize.pow(16);
    let mut count_sum = HashMap::new();
    while num < 10_usize.pow(9) {
        let count = count_sum.entry(digit_sum(num)).or_insert(0);
        *count += 1;
        num += 10;
    }
    println!("{:?}", count_sum);
    //while count <= N {
    //    count += vv[digit_sum(num)];
    //    num += 10;
    //}
    let mut flag = 0;
    assert_eq!(count, 0);
    num = 0;
    while count <= N {
        for (key, val) in count_sum.iter() {
            count += vv[digit_sum(flag) + key] * val;
        }
        num += 10_usize.pow(9);
        flag += 1;
    }
    while count >= N {
        num -= 1;
        if primes.contains(&(digit_sum(num))) {
            count -= 1;
        }
    }
    assert_eq!(num, 45009328011709400);
    println!("{:?}", now.elapsed());
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
    while count >= n {
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

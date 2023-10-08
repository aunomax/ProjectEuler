/// # Smallest Multiple
/// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
///
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to
/// 20?
fn main() {
    let s: u64 = 20;
    let primes = primes(s);
    let mut tmp = vec![0; primes.len()];
    for n in 1..s + 1 {
        larger_divisor(&mut tmp, prime_divisor(n, &primes));
        println!("{:?}", tmp);
    }
    let mut multi: u64 = 1;
    for i in 0..primes.len() {
        multi *= primes[i].pow(tmp[i].try_into().unwrap());
    }

    println!("{:?}", primes);
    assert_eq!(prime_divisor(10, &primes), [1, 0, 1, 0, 0, 0, 0, 0]);
    println!("{}", multi);
    assert_eq!(232792560, multi);
}

fn is_prime(n: u64) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

fn primes(n: u64) -> Vec<u64> {
    let mut p = vec![];
    if n <= 1 {
        return p;
    }
    for i in 2..n + 1 {
        if is_prime(i) {
            p.push(i);
        }
    }
    return p;
}

fn prime_divisor(n: u64, primes: &Vec<u64>) -> Vec<u64> {
    let mut num = vec![];
    for p in primes {
        let mut count = 0;
        let mut tmp = n;
        while tmp % p == 0 {
            count += 1;
            tmp /= p;
        }
        num.push(count);
    }
    return num;
}

fn larger_divisor(d1: &mut Vec<u64>, d2: Vec<u64>) {
    if d1.len() != d2.len() {
        panic!("two divisor lists should have the same length.");
    }
    for i in 0..d1.len() {
        if d1[i] < d2[i] {
            d1[i] = d2[i];
        }
    }
}

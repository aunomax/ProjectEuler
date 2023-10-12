/// # Longest Collatz Sequence
///
///

pub fn main() {
    const N: u64 = 1000000;
    let mut num = 1;
    let mut flag = 0;
    for i in 1..N + 1 {
        let tmp = chain(i);
        if tmp > flag {
            flag = tmp;
            num = i;
        }
    }
    assert_eq!(837799, num);
}

pub fn collatz(n: u64) -> u64 {
    if n % 2 == 0 {
        return n / 2;
    } else {
        return 3 * n + 1;
    }
}

pub fn chain(n: u64) -> u64 {
    let mut tmp = 1;
    let mut m = n;
    while m != 1 {
        m = collatz(m);
        tmp += 1;
    }
    return tmp;
}

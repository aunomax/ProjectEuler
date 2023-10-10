/// # Highly Divisible Triangular Number
///
/// 76576500

fn main() {
    const STOP: usize = 500;
    let mut n = STOP / 2;
    while tri_div_num(n) <= STOP {
        n += 1;
    }
    assert_eq!(76576500, (n + 1) * n / 2);
}

pub fn div_num(n: usize, stop: usize) -> usize {
    let mut count: usize = 0;
    for i in 1..stop + 1 {
        if n % i == 0 {
            count += 1;
        }
    }
    return 2 * count;
}

pub fn tri_div_num(n: usize) -> usize {
    let m = ((n + 1) * n) / 2;
    if n % 2 == 0 {
        return div_num(m, n / 2);
    } else {
        return div_num(m, (n + 1) / 2);
    }
}

/// # Lattice Paths
///
/// Starting in the top left corner of a 2*2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
///
/// How many such routes are there through a 20*20 grid?
///

fn main() {
    const N: u64 = 20;
    assert_eq!(137846528820, combination(N * 2, N));
}

pub fn combination(m: u64, n: u64) -> u64 {
    if m < n {
        return 1;
    }
    let tmp = if n > m - n { m - n } else { n };
    let mut comb = 1;
    for i in 0..tmp {
        comb *= m - i;
        comb /= i + 1;
    }
    return comb;
}

/// Special Pythagorean Triplet
///
/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
/// a^2 + b^2 = c^2.
/// For example, 3^2 + 4^2 = 9 + 16 = 25.
///
/// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
/// Find the product abc.

fn main() {
    const N: usize = 1000;
    let mut flag = false;
    for i in 1..N - 1 {
        for j in i..N - i - 1 {
            let k = N - i - j;
            if is_triplet(i, j, k) && is_pythagorean(i, j, k) {
                println!("{}", i * j * k);
                flag = true;
            }
            if flag {
                break;
            }
        }
        if flag {
            break;
        }
    }
}

fn is_triplet(a: usize, b: usize, c: usize) -> bool {
    return a + b > c && a + c > b && b + c > a;
}

fn is_pythagorean(a: usize, b: usize, c: usize) -> bool {
    return a.pow(2) + b.pow(2) == c.pow(2);
}

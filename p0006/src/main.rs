fn main() {
    let n: u64 = 100;
    let mut sum = 0;
    let mut square = 0;
    for i in 1..n + 1 {
        sum += i;
        square += i.pow(2);
    }
    assert_eq!(25164150, sum.pow(2) - square);
}

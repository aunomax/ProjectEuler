fn main() {
    let mut fib1 = 1;
    let mut fib2 = 2;
    let mut sum = 0;
    while fib2 <= 4000000 {
        if fib1 % 2 == 0 {
            sum += fib1;
        }
        if fib2 % 2 == 0 {
            sum += fib2;
        }
        fib1 += fib2;
        fib2 += fib1;
    }
    println!("{}", sum);
}

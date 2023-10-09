/// # Summation of Primes
///
/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
///
/// Find the sum of all the primes below two million.

fn main() {
    const N: usize = 2000000;
    let mut num = 3;
    let mut sum = 2;
    let mut primes = vec![2];
    while num < N {
        let mut j = 0;
        let mut flag = true;
        while primes[j] * primes[j] <= num {
            if num % primes[j] == 0 {
                flag = false;
                break;
            }
            j += 1;
        }
        if flag {
            primes.push(num);
            sum += num;
        }
        num += 1;
    }
    assert_eq!(142913828922, sum);
}

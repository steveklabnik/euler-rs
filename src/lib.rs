
use std::iter::range_step;

pub fn p1() -> u32 {
    range(0, 1_000).fold(0, |a, b| if b % 3 == 0 || b % 5 == 0 { a + b } else { a })
}
pub fn p2() -> u32 {
    fn fib(a: u32, b: u32) -> u32 {
        let c = a + b;
        if c > 4_000_000 { return 0; }
        let f = fib(b, c);
        if c % 2 == 0 { f + c } else { f }
    }
    fib(0, 1)
}
pub fn p3() -> u64 {
    let num = 600_851_475_143u64;
    let sqrt = (num as f64).sqrt();
    let size = sqrt as uint;
    let mut sieve = Vec::from_elem(size, true);
    *sieve.get_mut(0) = false;
    *sieve.get_mut(1) = false;
    let sqrtsqrt = sqrt.sqrt() as uint;
    for i in range(2, sqrtsqrt) {
        if !sieve[i] {
            for j in range_step(i * i, size, i) {
                *sieve.get_mut(j) = false;
            }
        }
    }
    let mut cur = num;
    for i in range(2, size) {
        if !sieve[i] { continue }
        let val = i as u64;
        if cur % val != 0 { continue }
        if cur == val { return val }
        cur = cur / val
    }
    num
}

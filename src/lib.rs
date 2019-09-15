pub mod combinadics;
pub mod factoradics;

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    debug_assert!(a > 0 || b > 0, "Failed precondition");
    while b > 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

#[test]
fn gcd_ok() {
    fn test(a: usize, b: usize, r: usize) {
        assert_eq!(gcd(a, b), r, "gcd({}, {})", a, b);
        assert_eq!(gcd(b, a), r, "gcd({}, {})", b, a);
    }
    for i in 1 .. 10 {
        test(0, i, i);
    }
    test(12, 8, 4);
    test(12, 18, 6);
    test(24, 8, 8);
    test(24, 18, 6);
    test(72, 18, 18);
}

pub fn fact(mut n: usize) -> usize {
    let mut r = 1;
    while n > 0 {
        r *= n;
        n -= 1;
    }
    r
}

#[test]
fn fact_ok() {
    fn test(n: usize, r: usize) {
        assert_eq!(fact(n), r, "fact({})", n);
    }
    test(0, 1);
    test(1, 1);
    test(2, 2);
    test(3, 6);
    test(4, 24);
    test(5, 120);
}

pub fn combination(n: usize, k: usize) -> usize {
    if n < k {
        return 0;
    }
    let mut r = 1;
    let mut d = fact(k);
    for i in 0 .. k {
        let mut m = n - i;
        if d > 1 {
            let g = gcd(m, d);
            m /= g;
            d /= g;
        }
        r *= m;
    }
    debug_assert_eq!(d, 1);
    r
}

#[test]
fn combination_ok() {
    fn test(n: usize, k: usize, r: usize) {
        assert_eq!(combination(n, k), r, "combination({}, {})", n, k);
    }
    test(0, 0, 1);
    test(1, 0, 1);
    test(2, 0, 1);
    test(0, 1, 0);
    test(0, 2, 0);
    test(3, 2, 3);
    test(4, 2, 6);
}

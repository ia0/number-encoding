fn is_ordered_set<T: Ord>(xs: &[T]) -> bool {
    xs.windows(2).all(|w| w[0] < w[1])
}

#[test]
fn is_ordered_set_ok() {
    fn test(xs: &[usize], r: bool) {
        assert_eq!(is_ordered_set(xs), r, "is_ordered_set({:?})", xs);
    }
    test(&[0, 1, 2], true);
    test(&[0, 1, 1], false);
    test(&[0, 2, 1], false);
}

fn is_unordered_set<T: Ord>(xs: &[T]) -> bool {
    let mut xs: Vec<&T> = xs.iter().collect();
    xs.sort();
    is_ordered_set(&xs)
}

#[test]
fn is_unordered_set_ok() {
    fn test(xs: &[usize], r: bool) {
        assert_eq!(is_unordered_set(xs), r, "is_unordered_set({:?})", xs);
    }
    test(&[0, 1, 2], true);
    test(&[0, 1, 1], false);
    test(&[0, 2, 1], true);
}

pub fn decode_mut<T: Ord>(xs: &mut [T], mut p: usize) {
    assert!(is_ordered_set(xs));
    let n = xs.len();
    let mut ps = Vec::with_capacity(n);
    for i in 1 ..= n {
        ps.push(p % i);
        p /= i;
    }
    assert_eq!(p, 0, "Failed precondition");
    for (i, &p) in ps.iter().rev().enumerate() {
        xs[i ..= i + p].rotate_right(1);
    }
}

pub fn decode<T: Clone + Ord>(xs: &[T], p: usize) -> Vec<T> {
    let mut xs = xs.to_vec();
    decode_mut(&mut xs, p);
    xs
}

#[test]
fn decode_ok() {
    fn test(d: usize, p: usize, e: &[usize]) {
        let mut r = Vec::with_capacity(d);
        for i in 0 .. d {
            r.push(i);
        }
        decode_mut(&mut r, p);
        assert_eq!(r, e, "decode({})", p);
    }
    test(3, 0, &[0, 1, 2]);
    test(3, 1, &[0, 2, 1]);
    test(3, 2, &[1, 0, 2]);
    test(3, 3, &[1, 2, 0]);
    test(3, 4, &[2, 0, 1]);
    test(3, 5, &[2, 1, 0]);
}

pub fn encode<T: Ord>(xs: &[T]) -> usize {
    assert!(is_unordered_set(xs), "Failed precondition");
    let n = xs.len();
    let mut ps = Vec::with_capacity(n);
    for i in 0 .. n {
        ps.push(xs[i + 1 ..].iter().filter(|&x| x < &xs[i]).count());
    }
    let mut r = 0;
    let mut k = 1;
    for (i, &p) in ps.iter().rev().enumerate() {
        r += p * k;
        k *= i + 1;
    }
    r
}

#[test]
fn encode_ok() {
    fn test(xs: &[usize], p: usize) {
        assert_eq!(encode(xs), p, "encode({:?})", xs);
    }
    test(&[0, 1, 2], 0);
    test(&[0, 2, 1], 1);
    test(&[1, 0, 2], 2);
    test(&[1, 2, 0], 3);
    test(&[2, 0, 1], 4);
    test(&[2, 1, 0], 5);
}

#[test]
fn decode_encode_bij() {
    for p in 0 .. 24 {
        assert_eq!(encode(&decode(&[0, 1, 2, 3], p)), p);
    }
}

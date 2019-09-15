pub fn encode(xs: &[usize]) -> usize {
    let mut r = 0;
    for (i, &x) in xs.iter().enumerate() {
        r += crate::combination(x, i + 1);
    }
    r
}

#[test]
fn encode_ok() {
    fn test(xs: &[usize], r: usize) {
        assert_eq!(encode(xs), r, "encode({:?})", xs);
    }
    test(&[0, 1, 2], 0);
    test(&[0, 1, 3], 1);
    test(&[0, 2, 3], 2);
    test(&[1, 2, 3], 3);
    test(&[0, 1, 4], 4);
    test(&[0, 2, 4], 5);
    test(&[1, 2, 4], 6);
    test(&[0, 3, 4], 7);
    test(&[1, 3, 4], 8);
    test(&[2, 3, 4], 9);
    test(&[0, 1, 5], 10);
}

pub fn decode_mut(mut n: usize, mut k: usize, r: &mut [usize]) {
    debug_assert_eq!(r.len(), k, "Failed precondition");
    while k > 0 {
        let mut i = k;
        let mut x = 1;
        while x <= n {
            i += 1;
            x *= i;
            x /= i - k;
        }
        x *= i - k;
        x /= i;
        i -= 1;
        n -= x;
        k -= 1;
        r[k] = i;
    }
}

pub fn decode(n: usize, k: usize) -> Vec<usize> {
    let mut r = vec![0; k];
    decode_mut(n, k, &mut r);
    r
}

#[test]
fn decode_ok() {
    fn test(n: usize, k: usize, r: &[usize]) {
        assert_eq!(decode(n, k), r, "decode({}, {})", n, k);
    }
    test(0, 3, &[0, 1, 2]);
    test(1, 3, &[0, 1, 3]);
    test(2, 3, &[0, 2, 3]);
    test(3, 3, &[1, 2, 3]);
    test(4, 3, &[0, 1, 4]);
    test(5, 3, &[0, 2, 4]);
    test(6, 3, &[1, 2, 4]);
    test(7, 3, &[0, 3, 4]);
    test(8, 3, &[1, 3, 4]);
    test(9, 3, &[2, 3, 4]);
    test(10, 3, &[0, 1, 5]);
}

pub struct Iter {
    decoded: Vec<usize>,
    encoded: usize,
}

impl Iter {
    pub fn new(k: usize) -> Iter {
        let mut result = Iter { decoded: Vec::new(), encoded: 0 };
        for i in 0 .. k {
            result.decoded.push(i);
        }
        result
    }

    pub fn get_decoded(&self) -> &[usize] {
        &self.decoded
    }

    pub fn get_encoded(&self) -> usize {
        self.encoded
    }

    pub fn advance(&mut self) {
        let k = self.decoded.len();
        for i in 0 .. k {
            self.decoded[i] += 1;
            if i == k - 1 || self.decoded[i] < self.decoded[i + 1] {
                break;
            }
            self.decoded[i] = i;
        }
        self.encoded += 1;
    }
}

#[test]
fn iter_ok() {
    fn test(k: usize, r: &[&[usize]]) {
        let mut iter = Iter::new(k);
        for (i, &r) in r.iter().enumerate() {
            assert_eq!(iter.get_decoded(), r);
            assert_eq!(iter.get_encoded(), i);
            assert_eq!(encode(r), i);
            iter.advance();
        }
    }
    test(0, &[&[]]);
    test(1, &[&[0], &[1], &[2]]);
    test(2, &[&[0, 1], &[0, 2], &[1, 2], &[0, 3], &[1, 3], &[2, 3]]);
    test(
        3,
        &[
            &[0, 1, 2],
            &[0, 1, 3],
            &[0, 2, 3],
            &[1, 2, 3],
            &[0, 1, 4],
            &[0, 2, 4],
            &[1, 2, 4],
            &[0, 3, 4],
            &[1, 3, 4],
            &[2, 3, 4],
            &[0, 1, 5],
            &[0, 2, 5],
            &[1, 2, 5],
        ],
    );
}

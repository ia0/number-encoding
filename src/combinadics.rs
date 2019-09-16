// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Combinatorial number system
//!
//! See [wikipedia] for more information.
//!
//! [wikipedia]: https://en.wikipedia.org/wiki/Combinatorial_number_system

/// Writes the combination of a value to a slice.
///
/// The written combination can be encoded with [`encode`] to get back `n`.
///
/// ```rust
/// # use number_encoding::combinadics::{decode_mut, encode};
/// # let n = 5;
/// # let k = 3;
/// let mut xs = vec![0; k];
/// decode_mut(n, k, &mut xs);
/// assert_eq!(encode(&xs), n);
/// ```
///
/// See [`decode`] for a version that allocates a vector for the combination.
///
/// # Panics
///
/// Panics in debug mode if `n > 0 && k == 0`.
///
/// [`decode`]: fn.decode.html
/// [`encode`]: fn.encode.html
pub fn decode_mut(mut n: usize, mut k: usize, r: &mut [usize]) {
    debug_assert_eq!(r.len(), k, "Failed precondition");
    debug_assert!(k > 0 || n == 0, "Failed precondition");
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

/// Returns the combination of a value.
///
/// The returned combination can be encoded with [`encode`] to get back `n`.
///
/// ```rust
/// # use number_encoding::combinadics::{decode, encode};
/// let n = 5;
/// let k = 3;
/// let xs = decode(n, k);
/// assert_eq!(encode(&xs), 5);
/// ```
///
/// See [`decode_mut`] for a version that writes the combination to a provided slice.
///
/// # Panics
///
/// Panics in debug mode if `n > 0 && k == 0`.
///
/// # Examples
///
/// ```rust
/// # use number_encoding::combinadics::decode;
/// assert_eq!(decode(0, 3), &[0, 1, 2]);
/// assert_eq!(decode(1, 3), &[0, 1, 3]);
/// assert_eq!(decode(2, 3), &[0, 2, 3]);
/// ```
///
/// [`decode_mut`]: fn.decode_mut.html
/// [`encode`]: fn.encode.html
pub fn decode(n: usize, k: usize) -> Vec<usize> {
    let mut r = vec![0; k];
    decode_mut(n, k, &mut r);
    r
}

#[test]
fn decode_ok() {
    fn test(n: usize, k: usize, r: &[usize]) {
        assert_eq!(decode(n, k), r, "n={} k={}", n, k);
    }
    test(0, 0, &[]);
    test(0, 1, &[0]);
    test(1, 1, &[1]);
    test(2, 1, &[2]);
    test(0, 2, &[0, 1]);
    test(1, 2, &[0, 2]);
    test(2, 2, &[1, 2]);
    test(3, 2, &[0, 3]);
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

/// Returns the value of a combination.
///
/// The returned value can be decoded with [`decode`] to get back `xs`.
///
/// ```rust
/// # use number_encoding::combinadics::{decode, encode};
/// # let xs = &[0, 2, 4];
/// let k = xs.len();
/// let n = encode(xs);
/// assert_eq!(decode(n, k), xs);
/// ```
///
/// # Panics
///
/// Panics in debug mode if `xs` is not increasing.
///
/// # Examples
///
/// ```rust
/// # use number_encoding::combinadics::encode;
/// assert_eq!(encode(&[0, 1, 2]), 0);
/// assert_eq!(encode(&[0, 1, 3]), 1);
/// assert_eq!(encode(&[0, 2, 3]), 2);
/// assert_eq!(encode(&[1, 2, 3]), 3);
/// assert_eq!(encode(&[0, 1, 4]), 4);
/// assert_eq!(encode(&[0, 2, 4]), 5);
/// ```
///
/// [`decode`]: fn.decode.html
pub fn encode(xs: &[usize]) -> usize {
    debug_assert!(crate::is_ordered_set(xs), "Failed precondition");
    let mut r = 0;
    for (i, &x) in xs.iter().enumerate() {
        r += crate::combination(x, i + 1);
    }
    r
}

#[test]
fn encode_ok() {
    fn test(xs: &[usize], r: usize) {
        assert_eq!(encode(xs), r, "xs={:?}", xs);
    }
    test(&[], 0);
    test(&[0], 0);
    test(&[1], 1);
    test(&[0, 1], 0);
    test(&[0, 2], 1);
    test(&[1, 2], 2);
    test(&[0, 3], 3);
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

/// Iterates over all k-combinations in value order.
///
/// # Examples
///
/// To iterate over all `k`-combinations in a set of `n` elements:
///
/// ```rust
/// # use number_encoding::combinadics::{Iter, encode};
/// # use number_encoding::combination;
/// # fn process(xs: &[usize]) {}
/// # let n = 5;
/// # let k = 3;
/// let mut iter = Iter::new(k);
/// for i in 0 .. combination(n, k) {
///     let xs = iter.get();  // Returns the current k-combination.
///     assert_eq!(encode(xs), i);  // The k-combination are iterated in order.
///     process(xs);
///     iter.advance();  // Must be called after each iteration.
/// }
/// ```
pub struct Iter {
    data: Vec<usize>,
}

impl Iter {
    /// Constructs an iterator.
    pub fn new(k: usize) -> Iter {
        let mut data = Vec::new();
        for i in 0 .. k {
            data.push(i);
        }
        Iter { data }
    }

    /// Returns the current combination.
    ///
    /// The combination lifetime must not overlap with the call to [`advance`].
    ///
    /// [`advance`]: struct.Iter.html#method.advance
    pub fn get(&self) -> &[usize] {
        &self.data
    }

    /// Advances to the next combination.
    ///
    /// The lifetime of the current combination must be over when calling this method.
    pub fn advance(&mut self) {
        let k = self.data.len();
        for i in 0 .. k {
            self.data[i] += 1;
            if i == k - 1 || self.data[i] < self.data[i + 1] {
                break;
            }
            self.data[i] = i;
        }
    }
}

#[test]
fn iter_ok() {
    fn test(k: usize, r: &[&[usize]]) {
        let mut iter = Iter::new(k);
        for (i, &r) in r.iter().enumerate() {
            assert_eq!(iter.get(), r);
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

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

//! Factorial number system
//!
//! See [wikipedia] for more information.
//!
//! [wikipedia]: https://en.wikipedia.org/wiki/Factorial_number_system

/// Applies the permutation of the value `p` to the slice `xs`.
///
/// The applied permutation can be encoded with [`encode`] to get back `p`.
///
/// ```rust
/// # use number_encoding::factoradics::{decode_mut, encode};
/// # let mut xs = [0, 1, 2, 3];
/// # let p = 15;
/// decode_mut(&mut xs, p);
/// assert_eq!(encode(&xs), p);
/// ```
///
/// See [`decode`] for a version that allocates a vector for the permutation.
///
/// # Panics
///
/// Panics in debug mode if `xs` is not increasing.
///
/// [`decode`]: fn.decode.html
/// [`encode`]: fn.encode.html
pub fn decode_mut<T: Ord>(xs: &mut [T], mut p: usize) {
    debug_assert!(crate::is_ordered_set(xs), "Failed precondition");
    let n = xs.len();
    let mut ps = Vec::with_capacity(n);
    for i in 1 ..= n {
        ps.push(p % i);
        p /= i;
    }
    debug_assert_eq!(p, 0, "Failed precondition");
    for (i, &p) in ps.iter().rev().enumerate() {
        xs[i ..= i + p].rotate_right(1);
    }
}

/// Returns the permutation of the value `p` to the slice `xs`.
///
/// The returned permutation can be encoded with [`encode`] to get back `p`.
///
/// ```rust
/// # use number_encoding::factoradics::{decode, encode};
/// # let xs = [0, 1, 2, 3];
/// # let p = 15;
/// let xs = decode(&xs, p);
/// assert_eq!(encode(&xs), p);
/// ```
///
/// See [`decode_mut`] for a version that applies the permutation to the slice.
///
/// # Panics
///
/// Panics in debug mode if `xs` is not increasing.
///
/// # Examples
///
/// ```rust
/// # use number_encoding::factoradics::decode;
/// assert_eq!(decode(&[0, 1, 2], 0), &[0, 1, 2]);
/// assert_eq!(decode(&[0, 1, 2], 1), &[0, 2, 1]);
/// assert_eq!(decode(&[0, 1, 2], 2), &[1, 0, 2]);
/// ```
///
/// [`decode_mut`]: fn.decode_mut.html
/// [`encode`]: fn.encode.html
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
        assert_eq!(r, e, "p={}", p);
    }
    test(0, 0, &[]);
    test(1, 0, &[0]);
    test(2, 0, &[0, 1]);
    test(2, 1, &[1, 0]);
    test(3, 0, &[0, 1, 2]);
    test(3, 1, &[0, 2, 1]);
    test(3, 2, &[1, 0, 2]);
    test(3, 3, &[1, 2, 0]);
    test(3, 4, &[2, 0, 1]);
    test(3, 5, &[2, 1, 0]);
}

/// Returns the value of a permutation.
///
/// The returned value can be decoded with [`decode`] to get back `xs`.
///
/// ```rust
/// # use number_encoding::factoradics::{decode, encode};
/// # let xs = &[2, 0, 1];
/// let mut ys = xs.to_vec();
/// ys.sort();
/// let p = encode(xs);
/// assert_eq!(decode(&ys, p), xs);
/// ```
///
/// # Panics
///
/// Panics in debug mode if `xs` does not contain distinct elements.
///
/// # Examples
///
/// ```rust
/// # use number_encoding::factoradics::encode;
/// assert_eq!(encode(&[0, 1, 2]), 0);
/// assert_eq!(encode(&[0, 2, 1]), 1);
/// assert_eq!(encode(&[1, 0, 2]), 2);
/// ```
///
/// [`decode`]: fn.decode.html
pub fn encode<T: Ord>(xs: &[T]) -> usize {
    debug_assert!(crate::is_unordered_set(xs), "Failed precondition");
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
        assert_eq!(encode(xs), p, "xs={:?}", xs);
    }
    test(&[], 0);
    test(&[0], 0);
    test(&[0, 1], 0);
    test(&[1, 0], 1);
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

/// Iterates over all permutations of a slice.
///
/// The permutations are iterated in value order:
///
/// ```rust
/// # use number_encoding::factoradics::{Iter, encode};
/// # let mut xs = [0, 1, 2, 3];
/// let mut iter = Iter::new(&mut xs);
/// let mut i = 0;
/// while let Some(xs) = iter.next() {
///     assert_eq!(encode(xs), i);
///     i += 1;
/// }
/// ```
///
/// If the iteration goes to the end (i.e. [`next`] returns `None`), then the slice is restored to
/// its initial value (i.e. increasing):
///
/// ```rust
/// # use number_encoding::factoradics::Iter;
/// # let mut xs = [0, 1, 2, 3];
/// let saved_xs = xs.clone();
/// let mut iter = Iter::new(&mut xs);
/// while iter.next().is_some() {}
/// assert_eq!(xs, saved_xs);
/// ```
///
/// # Examples
///
/// ```rust
/// # use number_encoding::factoradics::Iter;
/// # fn process(xs: &[usize]) {}
/// # let mut xs = [0, 1, 2, 3];
/// let mut iter = Iter::new(&mut xs);
/// while let Some(xs) = iter.next() {
///     process(xs);
/// }
/// ```
///
/// [`next`]: struct.Iter.html#method.next
pub struct Iter<'a, T> {
    data: &'a mut [T],
    state: IterState,
}

enum IterState {
    New,
    Running,
    Done,
}

impl<'a, T: Ord> Iter<'a, T> {
    /// Constructs an iterator with an increasing slice.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if `xs` is not increasing.
    pub fn new(xs: &mut [T]) -> Iter<T> {
        debug_assert!(crate::is_ordered_set(xs));
        Iter { data: xs, state: IterState::New }
    }

    /// Returns the next permutation.
    ///
    /// If iteration is over, returns `None`.
    pub fn next(&mut self) -> Option<&[T]> {
        match self.state {
            IterState::New => self.state = IterState::Running,
            IterState::Running => {
                if self.advance() {
                    self.state = IterState::Done;
                }
            }
            IterState::Done => (),
        }
        match self.state {
            IterState::New => unreachable!(),
            IterState::Running => Some(&self.data),
            IterState::Done => None,
        }
    }

    fn advance(&mut self) -> bool {
        let k = self.data.len();
        if k == 0 {
            return true;
        }
        let mut i = k - 1;
        while i > 0 && self.data[i - 1] > self.data[i] {
            i -= 1;
        }
        if i == 0 {
            self.data.reverse();
            return true;
        }
        self.data[i ..].reverse();
        let j = self.data[i ..].iter().position(|x| x > &self.data[i - 1]).unwrap();
        self.data.swap(i - 1, i + j);
        false
    }
}

#[test]
fn iter_ok() {
    fn test(r: &[&[usize]]) {
        let mut xs = r[0].to_vec();
        let mut iter = Iter::new(&mut xs);
        let mut i = 0;
        while let Some(xs) = iter.next() {
            assert_eq!(xs, r[i]);
            assert_eq!(encode(xs), i);
            i += 1;
        }
        assert_eq!(r.len(), i);
        assert_eq!(xs, r[0]);
    }
    test(&[&[]]);
    test(&[&[0]]);
    test(&[&[0, 1], &[1, 0]]);
    test(&[&[0, 1, 2], &[0, 2, 1], &[1, 0, 2], &[1, 2, 0], &[2, 0, 1], &[2, 1, 0]]);
}

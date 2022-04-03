// Copyright 2022 Google LLC
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

//! Multinomial number system
//!
//! See [wikipedia] for more information.
//!
//! [wikipedia]: https://en.wikipedia.org/wiki/Multinomial_theorem#Multinomial_coefficients

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Applies the multiset permutation of the value `p` to the slice `xs`.
///
/// The applied multiset permutation can be encoded with [`encode`] to get back `p`.
///
/// ```rust
/// # use number_encoding::multinadics::{decode_mut, encode};
/// # let mut xs = [0, 0, 0, 1, 1, 2];
/// # let p = 15;
/// decode_mut(&mut xs, p);
/// assert_eq!(encode(&xs), p);
/// ```
///
/// See [`decode`] for a version that allocates a vector for the permutation.
///
/// # Panics
///
/// Panics in debug mode if `xs` is not non-decreasing or `p` is out of range.
pub fn decode_mut<T: Ord>(xs: &mut [T], mut p: usize) {
    let mut m = crate::multinomial(xs);
    debug_assert!(crate::is_ordered_multiset(xs), "Failed precondition");
    debug_assert!(p < m, "Failed precondition");
    let n = xs.len();
    for i in 0 .. n {
        let mut c = i;
        let mut k = 1;
        for j in i + 1 .. n {
            if xs[j] == xs[j - 1] {
                k += 1;
                continue;
            }
            let s = m * k / (n - i);
            if p < s {
                break;
            }
            p -= s;
            c = j;
            k = 1;
        }
        m = m * k / (n - i);
        xs[i ..= c].rotate_right(1);
    }
    debug_assert_eq!(m, 1);
    debug_assert_eq!(p, 0);
}

/// Returns the multiset permutation of the value `p` to the slice `xs`.
///
/// The returned multiset permutation can be encoded with [`encode`] to get back `p`.
///
/// ```rust
/// # use number_encoding::multinadics::{decode, encode};
/// # let xs = [0, 0, 0, 1, 1, 2];
/// # let p = 15;
/// let xs = decode(&xs, p);
/// assert_eq!(encode(&xs), p);
/// ```
///
/// See [`decode_mut`] for a version that applies the multiset permutation to the slice.
///
/// # Panics
///
/// Panics in debug mode if `xs` is not non-decreasing or `p` is out of range.
///
/// # Examples
///
/// ```rust
/// # use number_encoding::multinadics::decode;
/// assert_eq!(decode(&[0, 0, 1], 0), &[0, 0, 1]);
/// assert_eq!(decode(&[0, 0, 1], 1), &[0, 1, 0]);
/// assert_eq!(decode(&[0, 0, 1], 2), &[1, 0, 0]);
/// ```
#[cfg(feature = "alloc")]
pub fn decode<T: Clone + Ord>(xs: &[T], p: usize) -> Vec<T> {
    let mut xs = xs.to_vec();
    decode_mut(&mut xs[..], p);
    xs
}

#[test]
fn decode_ok() {
    fn test(xs: &[usize], p: usize, e: &[usize]) {
        let r = decode(xs, p);
        assert_eq!(r, e, "xs={xs:?} p={p}");
    }
    test(&[], 0, &[]);
    test(&[0], 0, &[0]);
    test(&[0, 1], 0, &[0, 1]);
    test(&[0, 1], 1, &[1, 0]);
    test(&[0, 0], 0, &[0, 0]);
    test(&[0, 0, 1], 0, &[0, 0, 1]);
    test(&[0, 0, 1], 1, &[0, 1, 0]);
    test(&[0, 0, 1], 2, &[1, 0, 0]);
    test(&[0, 0, 1, 1], 0, &[0, 0, 1, 1]);
    test(&[0, 0, 1, 1], 1, &[0, 1, 0, 1]);
    test(&[0, 0, 1, 1], 2, &[0, 1, 1, 0]);
    test(&[0, 0, 1, 1], 3, &[1, 0, 0, 1]);
    test(&[0, 0, 1, 1], 4, &[1, 0, 1, 0]);
    test(&[0, 0, 1, 1], 5, &[1, 1, 0, 0]);
    test(&[0, 0, 0, 1, 1, 2], 0, &[0, 0, 0, 1, 1, 2]);
    test(&[0, 0, 0, 1, 1, 2], 1, &[0, 0, 0, 1, 2, 1]);
    test(&[0, 0, 0, 1, 1, 2], 2, &[0, 0, 0, 2, 1, 1]);
    test(&[0, 0, 0, 1, 1, 2], 3, &[0, 0, 1, 0, 1, 2]);
    test(&[0, 0, 0, 1, 1, 2], 4, &[0, 0, 1, 0, 2, 1]);
    test(&[0, 0, 0, 1, 1, 2], 5, &[0, 0, 1, 1, 0, 2]);
    test(&[0, 0, 0, 1, 1, 2], 6, &[0, 0, 1, 1, 2, 0]);
    test(&[0, 0, 0, 1, 1, 2], 7, &[0, 0, 1, 2, 0, 1]);
    test(&[0, 0, 0, 1, 1, 2], 8, &[0, 0, 1, 2, 1, 0]);
    test(&[0, 0, 0, 1, 1, 2], 9, &[0, 0, 2, 0, 1, 1]);
    test(&[0, 0, 0, 1, 1, 2], 10, &[0, 0, 2, 1, 0, 1]);
}

/// Returns the value of a multiset permutation.
///
/// The returned value can be decoded with [`decode`] to get back `xs`.
///
/// ```rust
/// # use number_encoding::multinadics::{decode, encode};
/// # let xs = &[0, 1, 1, 0, 2, 0];
/// let mut ys = xs.to_vec();
/// ys.sort();
/// let p = encode(xs);
/// assert_eq!(decode(&ys, p), xs);
/// ```
///
/// # Examples
///
/// ```rust
/// # use number_encoding::multinadics::encode;
/// assert_eq!(encode(&[0, 0, 1]), 0);
/// assert_eq!(encode(&[0, 1, 0]), 1);
/// assert_eq!(encode(&[1, 0, 0]), 2);
/// ```
pub fn encode<T: Ord>(xs: &[T]) -> usize {
    let n = xs.len();
    let mut m = crate::multinomial(xs);
    let mut r = 0;
    for i in 0 .. n {
        for j in i + 1 .. n {
            if xs[j] >= xs[i] || xs[i + 1 .. j].contains(&xs[j]) {
                continue;
            }
            let k = xs[j ..].iter().filter(|&x| x == &xs[j]).count();
            r += m * k / (n - i);
        }
        let k = xs[i ..].iter().filter(|&x| x == &xs[i]).count();
        m = m * k / (n - i);
    }
    debug_assert_eq!(m, 1);
    r
}

#[test]
fn encode_ok() {
    fn test(xs: &[usize], p: usize) {
        assert_eq!(encode(xs), p, "xs={xs:?}");
    }
    test(&[], 0);
    test(&[0], 0);
    test(&[0, 1], 0);
    test(&[1, 0], 1);
    test(&[0, 0], 0);
    test(&[0, 0, 1], 0);
    test(&[0, 1, 0], 1);
    test(&[1, 0, 0], 2);
    test(&[0, 0, 1, 1], 0);
    test(&[0, 1, 0, 1], 1);
    test(&[0, 1, 1, 0], 2);
    test(&[1, 0, 0, 1], 3);
    test(&[1, 0, 1, 0], 4);
    test(&[1, 1, 0, 0], 5);
}

/// Iterates over all multiset permutations of a slice.
///
/// The multiset permutations are iterated in value order:
///
/// ```rust
/// # use number_encoding::multinadics::{Iter, encode};
/// # let mut xs = [0, 0, 0, 1, 1, 2];
/// let mut iter = Iter::new(&mut xs);
/// let mut i = 0;
/// while let Some(xs) = iter.next() {
///     assert_eq!(encode(xs), i);
///     i += 1;
/// }
/// ```
///
/// If the iteration goes to the end (i.e. [`next`](Iter::next) returns `None`), then the slice is
/// restored to its initial value (i.e. non-decreasing):
///
/// ```rust
/// # use number_encoding::multinadics::Iter;
/// # let mut xs = [0, 0, 0, 1, 1, 2];
/// let saved_xs = xs.clone();
/// let mut iter = Iter::new(&mut xs);
/// while iter.next().is_some() {}
/// assert_eq!(xs, saved_xs);
/// ```
///
/// # Examples
///
/// ```rust
/// # use number_encoding::multinadics::Iter;
/// # fn process(xs: &[usize]) {}
/// # let mut xs = [0, 0, 0, 1, 1, 2];
/// let mut iter = Iter::new(&mut xs);
/// while let Some(xs) = iter.next() {
///     process(xs);
/// }
/// ```
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
    /// Constructs an iterator with a non-decreasing slice.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if `xs` is not non-decreasing.
    pub fn new(xs: &mut [T]) -> Iter<T> {
        debug_assert!(crate::is_ordered_multiset(xs));
        Iter { data: xs, state: IterState::New }
    }

    /// Returns the next permutation.
    ///
    /// If iteration is over, returns `None`.
    #[allow(clippy::should_implement_trait)]
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
            IterState::Running => Some(self.data),
            IterState::Done => None,
        }
    }

    fn advance(&mut self) -> bool {
        let n = self.data.len();
        if n == 0 {
            return true;
        }
        let mut i = n - 1;
        while i > 0 && self.data[i - 1] >= self.data[i] {
            i -= 1;
        }
        if i == 0 {
            self.data.reverse();
            return true;
        }
        self.data[i ..].reverse();
        let k = self.data[i ..].iter().position(|x| x > &self.data[i - 1]).unwrap();
        self.data.swap(i - 1, i + k);
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
    test(&[&[0, 0, 1], &[0, 1, 0], &[1, 0, 0]]);
    test(&[&[0, 1, 2], &[0, 2, 1], &[1, 0, 2], &[1, 2, 0], &[2, 0, 1], &[2, 1, 0]]);
    test(&[
        &[0, 0, 1, 1],
        &[0, 1, 0, 1],
        &[0, 1, 1, 0],
        &[1, 0, 0, 1],
        &[1, 0, 1, 0],
        &[1, 1, 0, 0],
    ]);
}

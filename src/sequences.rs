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

//! Bit sequence number system
//!
//! This permits to convert between variable-length bit sequences (i.e. `[bool]`) and fixed-length
//! bit sequences (i.e. `usize`).

#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Maximum sequence length.
pub const MAX_LENGTH: usize = usize::BITS as usize - 1;

/// Maximum sequence value.
///
/// Sequences above this value are invalid.
pub const MAX_SEQUENCE: usize = usize::MAX - 1;

/// Returns the sequence length.
///
/// # Panics
///
/// Panics in debug mode if `s > MAX_SEQUENCE`.
pub fn decode_len(s: usize) -> usize {
    debug_assert!(s <= MAX_SEQUENCE, "Failed precondition");
    MAX_LENGTH - (s + 1).leading_zeros() as usize
}

#[test]
fn decode_len_ok() {
    fn test(s: usize, n: usize) {
        assert_eq!(decode_len(s), n, "s={s}");
    }
    test(0, 0);
    test(1, 1);
    test(2, 1);
    test(3, 2);
    test(6, 2);
    test(7, 3);
    test(14, 3);
}

/// Writes the sequence of a value to a slice.
///
/// The written sequence can be encoded with [`encode`] to get back `s`.
///
/// ```rust
/// # use number_encoding::sequences::{decode_len, decode_mut, encode};
/// # let s = 13;
/// let n = decode_len(s);
/// let mut xs = vec![false; n];
/// decode_mut(s, &mut xs);
/// assert_eq!(encode(&xs), s);
/// ```
///
/// See [`decode`] for a version that allocates a vector for the sequence.
///
/// # Panics
///
/// Panics in debug mode if `s > MAX_SEQUENCE` or `xs.len() != decode_len(s)`.
pub fn decode_mut(s: usize, xs: &mut [bool]) {
    debug_assert!(s <= MAX_SEQUENCE, "Failed precondition");
    let n = decode_len(s);
    debug_assert_eq!(xs.len(), n, "Failed precondition");
    for (i, x) in xs.iter_mut().rev().enumerate() {
        *x = (s + 1) & 1 << i != 0;
    }
}

/// Returns the sequence of a value.
///
/// The returned sequence can be encoded with [`encode`] to get back `s`.
///
/// ```rust
/// # use number_encoding::sequences::{decode, encode};
/// let s = 13;
/// let xs = decode(s);
/// assert_eq!(encode(&xs), s);
/// ```
///
/// See [`decode_mut`] for a version that writes the sequence to a provided slice.
///
/// # Panics
///
/// Panics in debug mode if `s > MAX_SEQUENCE`.
///
/// # Examples
///
/// ```rust
/// # use number_encoding::sequences::decode;
/// assert_eq!(decode(0), &[]);
/// assert_eq!(decode(1), &[false]);
/// assert_eq!(decode(2), &[true]);
/// assert_eq!(decode(13), &[true, true, false]);
/// ```
#[cfg(feature = "alloc")]
pub fn decode(s: usize) -> Vec<bool> {
    let n = decode_len(s);
    let mut xs = vec![false; n];
    decode_mut(s, &mut xs);
    xs
}

#[test]
fn decode_ok() {
    fn test(s: usize, xs: &[bool]) {
        assert_eq!(decode(s), xs, "s={s} xs={xs:?}");
    }
    test(0, &[]);
    test(1, &[false]);
    test(2, &[true]);
    test(3, &[false, false]);
    test(4, &[false, true]);
    test(5, &[true, false]);
    test(6, &[true, true]);
    test(7, &[false, false, false]);
    test(8, &[false, false, true]);
    test(9, &[false, true, false]);
    test(10, &[false, true, true]);
    test(11, &[true, false, false]);
    test(12, &[true, false, true]);
    test(13, &[true, true, false]);
    test(14, &[true, true, true]);
}

/// Returns the value of a sequence.
///
/// The returned value can be decoded with [`decode`] to get back `xs`.
///
/// ```rust
/// # use number_encoding::sequences::{decode, encode};
/// # let xs = &[true, true, false];
/// let s = encode(xs);
/// assert_eq!(decode(s), xs);
/// ```
///
/// # Panics
///
/// Panics in debug mode if `xs.len() > MAX_LENGTH`.
///
/// # Examples
///
/// ```rust
/// # use number_encoding::sequences::encode;
/// assert_eq!(encode(&[]), 0);
/// assert_eq!(encode(&[false]), 1);
/// assert_eq!(encode(&[true]), 2);
/// assert_eq!(encode(&[true, true, false]), 13);
/// ```
pub fn encode(xs: &[bool]) -> usize {
    debug_assert!(xs.len() <= MAX_LENGTH, "Failed precondition");
    let mut s = 0;
    for &x in xs {
        s = 2 * s + 1 + x as usize;
    }
    s
}

#[test]
fn encode_ok() {
    fn test(xs: &[bool], s: usize) {
        assert_eq!(encode(xs), s, "xs={xs:?}");
    }
    test(&[], 0);
    test(&[false], 1);
    test(&[true], 2);
    test(&[false, false], 3);
    test(&[false, true], 4);
    test(&[true, false], 5);
    test(&[true, true], 6);
    test(&[false, false, false], 7);
    test(&[false, false, true], 8);
    test(&[false, true, false], 9);
    test(&[false, true, true], 10);
    test(&[true, false, false], 11);
    test(&[true, false, true], 12);
    test(&[true, true, false], 13);
    test(&[true, true, true], 14);
}

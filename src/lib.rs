// Copyright 2019-2022 Google LLC
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

//! Number systems
//!
//! This crate provides number systems for combinations, factorials, multinomials, and sequences of
//! bits.

#![no_std]
#![warn(unused_results, missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod combinadics;
pub mod factoradics;
pub mod multinadics;
pub mod sequences;

/// Returns the greatest common divisor of `a` and `b`.
///
/// See [wikipedia] for more information.
///
/// # Panics
///
/// Panics in debug mode if `a == 0 && b == 0`.
///
/// # Examples
///
/// ```rust
/// # use number_encoding::greatest_common_divisor;
/// assert_eq!(greatest_common_divisor(2, 3), 1);
/// assert_eq!(greatest_common_divisor(5, 1), 1);
/// assert_eq!(greatest_common_divisor(5, 5), 5);
/// assert_eq!(greatest_common_divisor(12, 8), 4);
/// assert_eq!(greatest_common_divisor(12, 18), 6);
/// ```
///
/// [wikipedia]: https://en.wikipedia.org/wiki/Greatest_common_divisor
pub fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    debug_assert!(a > 0 || b > 0, "Failed precondition");
    while b > 0 {
        a %= b;
        core::mem::swap(&mut a, &mut b);
    }
    a
}

#[test]
fn greatest_common_divisor_ok() {
    fn spec(a: usize, b: usize) -> usize {
        let mut r = 0;
        for i in 1 ..= std::cmp::max(a, b) {
            if a % i == 0 && b % i == 0 {
                r = std::cmp::max(r, i);
            }
        }
        r
    }
    for a in 0 .. 20 {
        for b in 0 .. 20 {
            if a == 0 && b == 0 {
                continue;
            }
            assert_eq!(greatest_common_divisor(a, b), spec(a, b), "a={a} b={b}");
        }
    }
}

/// Returns the factorial of `n`.
///
/// See [wikipedia] for more information.
///
/// # Examples
///
/// ```rust
/// # use number_encoding::factorial;
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(2), 2);
/// assert_eq!(factorial(3), 6);
/// assert_eq!(factorial(4), 24);
/// ```
///
/// [wikipedia]: https://en.wikipedia.org/wiki/Factorial
pub fn factorial(mut n: usize) -> usize {
    let mut r = 1;
    while n > 0 {
        r *= n;
        n -= 1;
    }
    r
}

#[test]
fn factorial_ok() {
    fn spec(n: usize) -> usize {
        if n == 0 {
            1
        } else {
            n * spec(n - 1)
        }
    }
    for n in 0 .. 10 {
        assert_eq!(factorial(n), spec(n), "n={n}");
    }
}

/// Returns the number of `k`-combinations of a set of `n` elements.
///
/// See [wikipedia] for more information.
///
/// # Examples
///
/// ```rust
/// # use number_encoding::combination;
/// assert_eq!(combination(4, 0), 1);
/// assert_eq!(combination(4, 1), 4);
/// assert_eq!(combination(4, 2), 6);
/// assert_eq!(combination(4, 3), 4);
/// assert_eq!(combination(4, 4), 1);
/// assert_eq!(combination(4, 5), 0);
/// ```
///
/// [wikipedia]: https://en.wikipedia.org/wiki/Combination
pub fn combination(n: usize, k: usize) -> usize {
    if n < k {
        return 0;
    }
    let mut r = 1;
    let mut d = factorial(k);
    for i in 0 .. k {
        let mut m = n - i;
        if d > 1 {
            let g = greatest_common_divisor(m, d);
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
    fn spec(n: usize, k: usize) -> usize {
        if k > n {
            0
        } else {
            factorial(n) / (factorial(n - k) * factorial(k))
        }
    }
    for n in 0 .. 5 {
        for k in 0 .. 5 {
            assert_eq!(combination(n, k), spec(n, k), "n={n} k={k}");
        }
    }
}

/// Returns the number of permutations of a multiset.
///
/// See [wikipedia] for more information.
///
/// # Examples
///
/// ```rust
/// # use number_encoding::multinomial;
/// assert_eq!(multinomial(&[2, 0, 1]), 6);
/// assert_eq!(multinomial(&[0, 1, 0]), 3);
/// assert_eq!(multinomial(&[0, 1, 1, 0, 2, 0]), 60);
/// ```
///
/// [wikipedia]: https://en.wikipedia.org/wiki/Multinomial_theorem#Number_of_unique_permutations_of_words
pub fn multinomial<T: Ord>(xs: &[T]) -> usize {
    let mut n = xs.len();
    let mut r = 1;
    for i in 0 .. xs.len() {
        if xs[.. i].contains(&xs[i]) {
            continue;
        }
        let k = xs[i ..].iter().filter(|&x| x == &xs[i]).count();
        r *= combination(n, k);
        n -= k;
    }
    r
}

#[test]
fn multinomial_ok() {
    fn test(xs: &[usize], r: usize) {
        assert_eq!(multinomial(xs), r, "xs={xs:?}");
    }
    test(&[], 1);
    test(&[0], 1);
    test(&[0, 0], 1);
    test(&[0, 1], 2);
    test(&[0, 1, 0], 3);
    test(&[0, 1, 0, 1], 6);
    test(&[0, 1, 1, 0, 2, 0], 60);
}

fn is_ordered_set<T: Ord>(xs: &[T]) -> bool {
    xs.windows(2).all(|w| w[0] < w[1])
}

#[test]
fn is_ordered_set_ok() {
    fn test(xs: &[usize], r: bool) {
        assert_eq!(is_ordered_set(xs), r, "xs={xs:?}");
    }
    test(&[0, 1], true);
    test(&[0, 0], false);
    test(&[1, 0], false);
}

// TODO(https://github.com/rust-lang/rust/issues/53485): Use is_sorted.
fn is_ordered_multiset<T: Ord>(xs: &[T]) -> bool {
    xs.windows(2).all(|w| w[0] <= w[1])
}

#[test]
fn is_ordered_multiset_ok() {
    fn test(xs: &[usize], r: bool) {
        assert_eq!(is_ordered_multiset(xs), r, "xs={xs:?}");
    }
    test(&[0, 1, 1], true);
    test(&[0, 0, 1], true);
    test(&[1, 0], false);
}

fn is_unordered_set<T: Ord>(xs: &[T]) -> bool {
    xs.iter().all(|x| xs.iter().filter(|&y| x == y).count() == 1)
}

#[test]
fn is_unordered_set_ok() {
    fn test(xs: &[usize], r: bool) {
        assert_eq!(is_unordered_set(xs), r, "xs={xs:?}");
    }
    test(&[0, 1], true);
    test(&[0, 0], false);
    test(&[1, 0], true);
}

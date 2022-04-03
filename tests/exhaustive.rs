use number_encoding::{combinadics, factoradics, factorial, multinadics, multinomial};

#[test]
fn factoradics_bijective() {
    use number_encoding::factoradics::{decode, encode};
    for n in 0 ..= 7 {
        let xs: Vec<_> = (0 .. n).collect();
        let m = factorial(n);
        for i in 0 .. m {
            assert_eq!(encode(&decode(&xs, i)), i);
        }
    }
}

#[test]
fn multinadics_bijective() {
    use number_encoding::multinadics::{decode, encode};
    for n in 0 ..= 7 {
        let mut iter = combinadics::Iter::new(n);
        loop {
            let mut xs = iter.get().to_vec();
            iter.advance();
            let mut p = 0;
            let mut skip = xs.first().map_or(false, |&x| x > 0);
            for i in 1 .. n {
                xs[i] -= i;
                match xs[i] - xs[p] {
                    0 if i - p < 3 => (),
                    1 => p = i,
                    _ => skip = true,
                }
            }
            let done = xs.last().map_or(true, |&x| 2 * x > n);
            match (skip, done) {
                (true, true) => break,
                (true, false) => continue,
                (false, _) => (),
            }
            let m = multinomial(&xs);
            for i in 0 .. m {
                assert_eq!(encode(&decode(&xs, i)), i);
            }
            if done {
                break;
            }
        }
    }
}

#[test]
fn factoradics_is_multinadics() {
    for n in 0 ..= 7 {
        let xs: Vec<_> = (0 .. n).collect();
        let m = factorial(n);
        assert_eq!(m, multinomial(&xs));
        for i in 0 .. m {
            let ys = factoradics::decode(&xs, i);
            assert_eq!(ys, multinadics::decode(&xs, i));
            let zs = factoradics::encode(&ys);
            assert_eq!(zs, multinadics::encode(&ys));
        }
    }
}

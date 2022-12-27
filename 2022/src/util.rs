type F<R> = fn() -> R;

pub struct Day<'a, T> {
    pub year: i32,
    pub day: i32,
    pub parts: &'a [F<T>],
    pub tests: &'a [F<()>],
}

pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    return if a > b { a } else { b };
}

pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    return if a > b { b } else { a };
}

pub struct FnIterator<I: Iterator, R, F: FnMut(I::Item) -> R> {
    inner: I,
    f: F,
}

impl<I: Iterator, R, F: FnMut(I::Item) -> R> FnIterator<I, R, F> {
    pub fn new(iter: I, f: F) -> Self {
        return Self { inner: iter, f };
    }
}

impl<I: Iterator, R, F: FnMut(I::Item) -> R> Iterator for FnIterator<I, R, F> {
    type Item = R;

    fn next(&mut self) -> Option<R> {
        if let Some(x) = self.inner.next() {
            let value = (self.f)(x);
            return Some(value);
        }
        return None;
    }
}

pub const fn is_prime(n: i64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i: i64 = 5;
    while (i * i) <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    return true;
}

pub const fn get_primes<const N: usize>() -> [i64; N] {
    let mut primes = [0; N];
    let mut n = 1;
    let mut i = 0;

    while n < i64::MAX {
        if is_prime(n) {
            primes[i] = n;
            i += 1;
        }

        if i == N {
            break;
        }

        n += 1;
    }

    return primes;
}

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

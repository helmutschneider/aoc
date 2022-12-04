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

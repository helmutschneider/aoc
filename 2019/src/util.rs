pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    return if a > b { a } else { b };
}

fn chmin<T: PartialOrd>(a: &mut T, b: T) {
    if *a > b {
        *a = b;
    }
}

fn func(n: u64) -> u64 {
    match n {
        0 => 0,
        _ => n + func(n - 1),
    }
}

fn fibo(n: u64) -> u64 {
    match n {
        // ベースケース
        0 => 0,
        1 => 1,
        // 再帰呼び出し
        _ => fibo(n - 1) + fibo(n - 2),
    }
}

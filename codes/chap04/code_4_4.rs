fn gcd(m: u64, n: u64) -> u64 {
    match n {
        // ベースケース
        0 => m,
        // 再帰呼び出し
        _ => gcd(n, m % n),
    }
}

fn main() {
    println!("{}", gcd(51, 15)); // 3 が出力される
    println!("{}", gcd(15, 51)); // 3 が出力される
}

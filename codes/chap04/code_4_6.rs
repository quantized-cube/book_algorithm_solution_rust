fn fibo(n: u64) -> u64 {
    println!("fibo({}) を呼び出しました", n);

    match n {
        // ベースケース
        0 => 0,
        1 => 1,
        // 再帰的に答えを求めて出力する
        _ => {
            let result = fibo(n - 1) + fibo(n - 2);
            println!("{} 項目 = {}", n, result);
            result
        }
    }
}

fn main() {
    fibo(6);
}

fn func(n: u64) -> u64 {
    // 再帰関数を呼び出したことを報告する
    println!("func({}) を呼び出しました", n);

    match n {
        0 => 0,
        _ => {
            // 再帰的に答えを求めて出力する
            let result = n + func(n - 1);
            println!("{} までの和 = {}", n, result);
            result
        }
    }
}

fn main() {
    func(5);
}

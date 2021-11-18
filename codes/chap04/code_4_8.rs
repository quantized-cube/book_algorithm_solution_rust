fn fibo(n: usize, memo: &mut Vec<i64>) -> i64 {
    // ベースケース
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    // メモをチェック (すでに計算済みならば答えをリターンする)
    if memo[n] != -1 {
        return memo[n];
    }

    // 答えをメモ化しながら，再帰呼び出し
    memo[n] = fibo(n - 1, memo) + fibo(n - 2, memo);
    return memo[n];
}

/// フィボナッチ数列を求める再帰関数をメモ化
fn main() {
    // fibo(n) の答えをメモ化する配列
    // メモ化用配列を -1 で初期化する
    let memo: &mut Vec<i64> = &mut vec![-1; 50];

    // fibo(49) をよびだす
    fibo(49, memo);

    // memo[0], ..., memo[49] に答えが格納されている
    for n in 2..50 {
        println!("{} 項目 = {}", n, memo[n]);
    }
}

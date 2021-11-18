use proconio::input;

fn chmin<T: PartialOrd>(a: &mut T, b: T) {
    if *a > b {
        *a = b;
    }
}

const INF: i64 = 1 << 60; // 十分大きな値 (ここでは 2^60 とする)

fn input() -> (usize, Vec<Vec<i64>>) {
    input! {
        n: usize,
        c: [[i64; n+1]; n+1],
    }
    (n, c)
}

fn main() {
    // 入力
    let (n, c) = input(use_default_values);

    // DP テーブル定義
    let mut dp = vec![INF; n + 1];

    // DP 初期条件
    dp[0] = 0;

    // DPループ
    for i in 0..=n {
        for j in 0..i {
            let b = dp[j] + c[j][i];
            chmin(&mut dp[i], b);
        }
    }

    // 答えの出力
    println!("{}", dp[n]);
}

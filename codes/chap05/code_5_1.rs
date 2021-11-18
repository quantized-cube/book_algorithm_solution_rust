use proconio::input;

const INF: i64 = 1 << 60; // 十分大きい値とする (ここでは 2^60)

fn input() -> (usize, Vec<i64>) {
    input! {
        n: usize,
        h: [i64; n],
    }
    (n, h)
}

fn main() {
    // 入力
    let (n, h) = input();

    // 配列 dp を定義 (配列全体を無限大を表す値に初期化)
    let mut dp = vec![INF; n];

    // 初期条件
    dp[0] = 0;

    // ループ
    for i in 1..n {
        if i == 1 {
            dp[i] = (h[i] - h[i - 1]).abs()
        } else {
            dp[i] = cmp::min(
                dp[i - 1] + (h[i] - h[i - 1]).abs(),
                dp[i - 2] + (h[i] - h[i - 2]).abs(),
            );
        }
    }

    // 答え
    println!("{}", dp[n - 1]);
}

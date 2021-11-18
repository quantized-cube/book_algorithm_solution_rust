use proconio::input;

fn chmin<T: PartialOrd>(a: &mut T, b: T) {
    if *a > b {
        *a = b;
    }
}

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

    // 初期化 (最小化問題なので INF に初期化)
    let mut dp = vec![INF; n];

    // 初期条件
    dp[0] = 0;

    // ループ
    for i in 0..n {
        if i + 1 < n {
            let b = dp[i] + (h[i] - h[i + 1]).abs();
            chmin(&mut dp[i + 1], b);
        }
        if i + 2 < n {
            let b = dp[i] + (h[i] - h[i + 2]).abs();
            chmin(&mut dp[i + 2], b);
        }
    }

    // 答え
    println!("{}", dp[n - 1]);
}

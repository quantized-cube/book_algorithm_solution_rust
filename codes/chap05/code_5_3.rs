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
    for i in 1..n {
        let (left, right) = dp.split_at_mut(i);
        let a = &mut right[0];
        let b = left[i - 1] + (h[i] - h[i - 1]).abs();
        chmin(a, b);
        if i > 1 {
            let b = left[i - 2] + (h[i] - h[i - 2]).abs();
            chmin(a, b);
        }
    }

    // 答え
    println!("{}", dp[n - 1]);
}

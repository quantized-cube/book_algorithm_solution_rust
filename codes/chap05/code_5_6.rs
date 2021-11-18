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

fn rec(i: usize, h: &Vec<i64>, dp: &mut Vec<i64>) -> i64 {
    // DP の値が更新されていたらそのままリターン
    if dp[i] < INF {
        return dp[i];
    }

    // ベースケース: 足場 0 のコストは 0
    if i == 0 {
        return 0;
    }

    // 答えを表す変数を INF で初期化する
    let mut res = INF;

    // 足場 i - 1 から来た場合
    let b = rec(i - 1, h, dp) + (h[i] - h[i - 1]).abs();
    chmin(&mut res, b);

    // 足場 i - 2 から来た場合
    if i > 1 {
        let b = rec(i - 2, h, dp) + (h[i] - h[i - 2]).abs();
        chmin(&mut res, b);
    }

    // 結果をメモしながら、返す
    dp[i] = res;
    res
}

fn main() {
    // 入力
    let (n, h) = input();

    // 初期化 (最小化問題なので INF に初期化)
    let mut dp = vec![INF; n];

    // 答え
    println!("{}", rec(n - 1, &h, &mut dp));
}

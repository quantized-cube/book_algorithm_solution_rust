use proconio::input;

fn chmin<T: PartialOrd>(a: &mut T, b: T) {
    if *a > b {
        *a = b;
    }
}

const INF: i64 = 1 << 29; // 十分大きな値 (ここでは 2^29 とする)

fn input() -> (String, String) {
    input! {
        s: String,
        t: String,
    }
    (s, t)
}

fn main() {
    // 入力
    let (s, t) = input();

    // DP テーブル定義
    let mut dp = vec![vec![INF; t.len() + 1]; s.len() + 1];

    // DP 初期条件
    dp[0][0] = 0;

    // DPループ
    for i in 0..s.len() + 1 {
        for j in 0..t.len() + 1 {
            // 変更操作
            if i > 0 && j > 0 {
                let b = dp[i - 1][j - 1];
                if s.as_bytes()[i - 1] == t.as_bytes()[j - 1] {
                    chmin(&mut dp[i][j], b);
                } else {
                    chmin(&mut dp[i][j], b + 1);
                }
            }

            // 削除操作
            if i > 0 {
                let b = dp[i - 1][j] + 1;
                chmin(&mut dp[i][j], b);
            };

            // 挿入操作
            if j > 0 {
                let b = dp[i][j - 1] + 1;
                chmin(&mut dp[i][j], b);
            };
        }
    }

    // 答えの出力
    println!("{}", dp[s.len()][t.len()]);
}

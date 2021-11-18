use proconio::input;

fn chmax<T: PartialOrd>(a: &mut T, b: T) {
    if *a < b {
        *a = b;
    }
}

fn input() -> (usize, usize, Vec<usize>, Vec<i64>) {
    input! {
        n: usize,
        w: usize,
        weight_value_pair: [(usize, i64); n],
    }
    let mut weight: Vec<usize> = Vec::new();
    let mut value: Vec<i64> = Vec::new();
    for item in weight_value_pair.into_iter() {
        weight.push(item.0);
        value.push(item.1);
    }
    (n, w, weight, value)
}

fn main() {
    // 入力
    let (n, w, weight, value) = input();

    // DP テーブル定義
    let mut dp = vec![vec![0; w + 1]; n + 1];

    // DPループ
    for i in 0..n {
        for u in 0..=w {
            // i 番目の品物を選ぶ場合
            if u >= weight[i] {
                let b = dp[i][u - weight[i]] + value[i];
                chmax(&mut dp[i + 1][u], b);
            }
            // i 番目の品物を選ばない場合
            let b = dp[i][u];
            chmax(&mut dp[i + 1][u], b);
        }
    }

    // 最適値の出力
    println!("{}", dp[n][w]);
}

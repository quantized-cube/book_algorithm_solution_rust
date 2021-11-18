use proconio::input;

fn input() -> (usize, i64, Vec<i64>, Vec<i64>) {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }
    (n, k, a, b)
}

const INF: i64 = 20000000; // 十分大きな値に

fn main() {
    let (n, k, a, b) = input();

    // 線形探索
    let mut min_value = INF;
    for i in 0..n {
        for j in 0..n {
            // 和が k 未満の場合はスキップ
            if a[i] + b[j] < k {
                continue;
            }

            // 最小値を更新
            if a[i] + b[j] < min_value {
                min_value = a[i] + b[j];
            }
        }
    }

    // 結果出力
    println!("{}", min_value);
}

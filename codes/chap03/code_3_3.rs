use proconio::input;

fn input() -> (usize, Vec<i64>) {
    input! {
        n: usize,
        a: [i64; n],
    }
    (n, a)
}

const INF: i64 = 20000000; // 十分大きな値に

fn main() {
    // 入力を受け取る
    let (n, a) = input();

    // 線形探索
    let mut min_value = INF;
    for i in 0..n {
        if a[i] < min_value {
            min_value = a[i];
        }
    }

    // 結果出力
    println!("{}", min_value);
}

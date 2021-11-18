use proconio::input;

fn input() -> (usize, i64, Vec<i64>) {
    input! {
        n: usize,
        v: i64,
        a: [i64; n],
    }
    (n, v, a)
}

fn main() {
    // 入力を受け取る
    let (n, v, a) = input();

    // 線形探索
    let mut exist = false; // 初期値は false に
    for i in 0..n {
        if a[i] == v {
            exist = true; // 見つかったらフラグを立てる
        }
    }

    // 結果出力
    if exist {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}

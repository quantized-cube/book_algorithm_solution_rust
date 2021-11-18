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
    let mut found_id = -1; // 初期値は -1 などありえない値に
    for i in 0..n {
        if a[i] == v {
            found_id = i as i64; // 見つかったら添字を記録
            break; // ループを抜ける
        }
    }

    // 結果出力 (-1 のときは見つからなかったことを表す)
    println!("{}", found_id);
}

use proconio::input;

// 区間を (i64, i64) で表す
type Interval = (i64, i64);

fn input() -> (usize, Vec<Interval>) {
    input! {
        n: usize,
        inter: [(i64, i64); n],
    }
    return (n, inter);
}

fn main() {
    // 入力
    let (n, mut inter) = input();

    // 終端時刻が早い順にソートする
    inter.sort_by_key(|a| a.1);

    // 貪欲に選ぶ
    let mut res = 0;
    let mut current_end_time = 0;
    for i in 0..n {
        // 最後に選んだ区間と被るのは除く
        if inter[i].0 < current_end_time {
            continue;
        }

        res += 1;
        current_end_time = inter[i].1;
    }
    println!("{}", res);
}

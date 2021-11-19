use proconio::input;

fn lower_bound<T>(v: &Vec<T>, key: T) -> T
where
    T: Ord + Copy,
{
    let mut left = 0usize;
    let mut right = v.len();
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if v[mid] >= key {
            right = mid;
        } else {
            left = mid;
        }
    }
    v[right]
}

const INF: i64 = 20000000; // 十分大きな値に

fn input() -> (usize, i64, Vec<i64>, Vec<i64>) {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }
    return (n, k, a, b);
}

fn main() {
    // 入力を受け取る
    let (n, k, a, b) = input();
    let mut b = b;

    // 暫定最小値を格納する変数
    let mut min_value = INF;

    // b をソート
    b.sort();

    // b に無限大を表す値 (INF) を追加しておく
    // これを行うことで、val = b.last() となる可能性を除外する
    b.push(INF);

    // a を固定して解く
    for i in 0..n {
        // b の中で k - a[i] 以上の範囲での最小値
        let val = lower_bound(&b, k - a[i]);

        // min_value と比較する
        if a[i] + val < min_value {
            min_value = a[i] + val;
        }
    }

    println!("{}", min_value);
}

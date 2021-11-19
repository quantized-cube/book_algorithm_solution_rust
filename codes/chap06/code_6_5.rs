use proconio::input;
use std::cmp;

fn input() -> (usize, Vec<usize>, Vec<usize>) {
    input! {
        n: usize,
        h_s_pair: [(usize, usize); n],
    }
    let mut h: Vec<usize> = Vec::new();
    let mut s: Vec<usize> = Vec::new();
    for item in h_s_pair.into_iter() {
        h.push(item.0);
        s.push(item.1);
    }
    return (n, h, s);
}

fn main() {
    // 入力
    let (n, h, s) = input();

    // 二分探索の上限値を求める
    let mut m: usize = 0;
    for i in 0..n {
        m = cmp::max(m, h[i] + s[i] * n);
    }

    // 二分探索
    let mut left = 0;
    let mut right = m;

    while right - left > 1 {
        let mid = left + (right - left) / 2;
        // 判定する
        let mut ok = true;
        let mut t = vec![0; n]; // 各風船を割るまでの制限時間
        for i in 0..n {
            // そもそも mid が初期高度より低かったら false
            if mid < h[i] {
                ok = false;
            } else {
                t[i] = (mid - h[i]) / s[i];
            }
        }

        // 時間制限がさし迫っている順にソート する
        t.sort();
        for i in 0..n {
            // 時間切れ発生の場合は false
            if t[i] < i {
                ok = false;
            }
        }

        if ok {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}

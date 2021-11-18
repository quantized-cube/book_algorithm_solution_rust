use proconio::input;

// 2 点 (x1, y1) と (x2, y2) との距離を求める関数
fn calc_dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    return ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
}

fn input() -> (usize, Vec<f64>, Vec<f64>) {
    input! {
        n: usize,
        x_y_pairs: [(f64, f64); n]
    }
    let mut x = Vec::new();
    let mut y = Vec::new();
    for x_y_pair in x_y_pairs {
        x.push(x_y_pair.0);
        y.push(x_y_pair.1);
    }
    (n, x, y)
}

fn main() {
    // 入力データを受け取る
    let (n, x, y) = input();

    // 求める値を、十分大きい値で初期化しておく
    let mut minimum_dist = 100000000.0;

    // 探索開始
    for i in 0..n {
        for j in i + 1..n {
            // (x[i], y[i]) と (x[j], y[j]) との距離
            let dist_i_j = calc_dist(x[i], y[i], x[j], y[j]);

            // 暫定最小値 minimum_dist を dist_i_j と比べる
            if dist_i_j < minimum_dist {
                minimum_dist = dist_i_j;
            }
        }
    }

    // 答えを出力する
    println!("{}", minimum_dist);
}

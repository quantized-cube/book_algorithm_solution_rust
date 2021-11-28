use proconio::input;

// ここでは重みを表す型を i64 型とする
#[derive(Clone)]
struct Edge {
    to: usize, // 隣接頂点番号
    w: i64,    // 重み
}

// 各頂点の隣接リストを，辺集合で表す
type Graph = Vec<Vec<Edge>>;

fn main() {
    input! {
        n: usize,
        m: usize,
        a_b_w: [(usize, usize, i64); m],
    }
    let mut g = vec![vec![]; m];
    for abw in a_b_w {
        g[abw.0].push(Edge {
            to: abw.1,
            w: abw.2,
        });
    }
}

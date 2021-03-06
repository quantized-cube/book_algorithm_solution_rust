use proconio::input;

type Graph = Vec<Vec<usize>>;

fn main() {
    input! {
        // 頂点数と辺数
        n: usize,
        m: usize,
        // グラフ
        a_b_pairs: [(usize, usize); m],
    }
    let mut g = vec![vec![]; m];
    for ab in a_b_pairs {
        g[ab.0].push(ab.1);
        // 無向グラフの場合は以下を追加
        // g[ab.1].push(ab.0);
    }
}

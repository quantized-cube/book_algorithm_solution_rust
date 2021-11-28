use proconio::input;

type Graph = Vec<Vec<usize>>;

// 深さ優先探索
fn dfs(g: &Graph, v: usize, seen: &mut Vec<bool>) {
    seen[v] = true; // v を訪問済にする

    // v から行ける各頂点 next_v について
    for next_v in &g[v] {
        if seen[*next_v] {
            continue; // next_v が探索済ならば探索しない
        }
        dfs(g, *next_v, seen); // 再帰的に探索
    }
}

fn input() -> (usize, usize, Graph) {
    input! {
        // 頂点数と辺数
        n: usize,
        m: usize,
        // グラフ入力受取 (ここでは有向グラフを想定)
        a_b_pairs: [(usize, usize); m],
    }
    let mut g = vec![vec![]; m];
    for ab in a_b_pairs {
        g[ab.0].push(ab.1);
    }
    (n, m, g)
}

fn main() {
    let (n, _m, g) = input();

    // 探索
    let mut seen = vec![false; n]; // 初期状態では全頂点が未訪問
    for v in 0..n {
        if seen[v] {
            continue; // すでに訪問済みなら探索しない
        }
        dfs(&g, v, &mut seen);
    }
}

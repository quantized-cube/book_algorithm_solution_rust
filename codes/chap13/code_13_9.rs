use proconio::input;

type Graph = Vec<Vec<usize>>;

// 木上の探索
fn dfs(
    g: &Graph,
    v: usize,
    p: i64,
    d: usize,
    depth: &mut Vec<usize>,
    subtree_size: &mut Vec<usize>,
) {
    depth[v] = d;
    for c in &g[v] {
        if *c as i64 == p {
            continue; // 探索が親方向へ逆流するのを防ぐ
        }
        dfs(g, *c, v as i64, d + 1, depth, subtree_size);
    }

    // 帰りがけ時に、部分木サイズを求める
    subtree_size[v] = 1; // 自分自身
    for c in &g[v] {
        if *c as i64 == p {
            continue;
        }

        // 子頂点を根とする部分きのサイズを加算する
        subtree_size[v] += subtree_size[*c];
    }
}

fn input() -> (usize, Graph) {
    input! {
        // 頂点数 (木なので辺数は N - 1 で確定)
        n: usize,
        // グラフ入力受取
        a_b_pairs: [(usize, usize); n - 1],
    }
    let mut g = vec![vec![]; n - 1];
    for ab in a_b_pairs {
        g[ab.0].push(ab.1);
        g[ab.1].push(ab.0);
    }
    (n, g)
}

fn main() {
    let (n, g) = input();

    // 探索
    let root = 0; // 仮に頂点 0 を根とする
    let mut depth = vec![0; n];
    let mut subtree_size = vec![0; n];
    dfs(&g, root, -1, 0, &mut depth, &mut subtree_size);

    // 結果
    for v in 0..n {
        println!(
            "{}: depth = {}, subtree_size = {}",
            v, depth[v], subtree_size[v]
        );
    }
}

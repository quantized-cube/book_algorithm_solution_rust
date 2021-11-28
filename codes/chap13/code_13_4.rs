use proconio::input;

type Graph = Vec<Vec<usize>>;

// 深さ優先探索
fn dfs(g: &Graph, v: usize, seen: &mut Vec<bool>) {
    seen[v] = true; // v を訪問済にする

    // v から行ける各頂点 next_v について
    for next_v in &g[v] {
        if seen[*next_v] {
            continue; // next_v が探索済だったらスルー
        }
        dfs(g, *next_v, seen); // 再帰的に探索
    }
}

fn input() -> (usize, usize, usize, usize, Graph) {
    input! {
        // 頂点数と辺数、s と t
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        // グラフ入力受取
        a_b_pairs: [(usize, usize); m],
    }
    let mut g = vec![vec![]; m];
    for ab in a_b_pairs {
        g[ab.0].push(ab.1);
    }
    (n, m, s, t, g)
}

fn main() {
    let (n, _m, s, t, g) = input();

    // 頂点 s をスタートとした探索
    let mut seen = vec![false; n]; // 全頂点を「未訪問」に初期化
    dfs(&g, s, &mut seen);

    // t にたどり着けるかどうか
    if seen[t] {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}

use proconio::input;

type Graph = Vec<Vec<usize>>;

// 二部グラフ判定
fn dfs(g: &Graph, v: usize, cur: i8, color: &mut Vec<i8>) -> bool {
    color[v] = cur;
    for next_v in &g[v] {
        // 隣接頂点がすでに色確定していた場合
        if color[*next_v] != -1 {
            // 同じ色が隣接した場合は二部グラフではない
            if color[*next_v] == cur {
                return false;
            }

            // 色が確定した場合には探索しない
            continue;
        }

        // 隣接頂点の色を変えて、再帰的に探索
        // false が返ってきたら false を返す
        if !dfs(g, *next_v, 1 - cur, color) {
            return false;
        }
    }
    return true;
}

fn input() -> (usize, usize, Graph) {
    input! {
        // 頂点数と辺数
        n: usize,
        m: usize,
        // グラフ入力受取
        a_b_pairs: [(usize, usize); m],
    }
    let mut g = vec![vec![]; m];
    for ab in a_b_pairs {
        g[ab.0].push(ab.1);
        g[ab.1].push(ab.0);
    }
    (n, m, g)
}

fn main() {
    let (n, _m, g) = input();

    // 探索
    let mut color = vec![-1; n];
    let mut is_bipartite = true;

    for v in 0..n {
        if color[v] != -1 {
            continue; // v が探索済みの場合は探索しない
        }
        if !dfs(&g, v, 0, &mut color) {
            is_bipartite = false;
        }
    }

    if is_bipartite {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}

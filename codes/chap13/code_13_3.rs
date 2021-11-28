use proconio::input;
use std::collections::VecDeque;

type Graph = Vec<Vec<usize>>;

// 入力: グラフ G と，探索の始点 s
// 出力: s から各頂点への最短路長を表す配列
fn bfs(g: &Graph, s: usize) -> Vec<i64> {
    let n = g.len(); // 頂点数
    let mut dist = vec![-1; n]; // 全頂点を「未訪問」に初期化
    let mut que: VecDeque<usize> = VecDeque::new();

    // 初期条件 (頂点 s を初期頂点とする)
    dist[s] = 0;
    que.push_back(s); // s を橙色頂点にする

    // BFS 開始 (キューが空になるまで探索を行う)
    while let Some(v) = que.pop_front() {
        // キューから先頭頂点を取り出す

        // v からたどれる頂点をすべて調べる
        for x in &g[v] {
            // すでに発見済みの頂点は探索しない
            if dist[*x] != -1 {
                continue;
            }

            // 新たな白色頂点 x について距離情報を更新してキューに挿入
            dist[*x] = dist[v] + 1;
            que.push_back(*x);
        }
    }
    dist
}

fn input() -> (usize, usize, Graph) {
    input! {
        // 頂点数と辺数
        n: usize,
        m: usize,
        // グラフ入力受取 (ここでは無向グラフを想定)
        a_b_pairs: [(usize, usize); m],
    }
    let mut g = vec![vec![]; m];
    for ab in a_b_pairs {
        g[ab.0].push(ab.1);
        g[ab.1].push(ab.0);
    }
    (n, m, g)
}

fn main() -> Vec<i64> {
    let (n, _m, g) = input();

    // 頂点 0 を始点とした BFS
    let dist = bfs(&g, 0);

    // 結果出力 (各頂点の頂点 0 からの距離を見る)
    for v in 0..n {
        println!("{}: {}", v, dist[v])
    }
}

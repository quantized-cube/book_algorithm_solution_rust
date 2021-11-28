use proconio::input;

type Graph = Vec<Vec<usize>>;

// トポロジカルソートする
fn rec(g: &Graph, v: usize, seen: &mut Vec<bool>, order: &mut Vec<usize>) {
    seen[v] = true;
    for next_v in &g[v] {
        if seen[*next_v] {
            continue; // すでに訪問済みなら探索しない
        }
        rec(g, *next_v, seen, order);
    }

    // v-out を記録する
    order.push(v);
}

fn input() -> (usize, usize, Graph) {
    input! {
        // 頂点数と枝数
        n: usize,
        m: usize,
        // 頂点数 N のグラフ
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
    let mut order = vec![]; // トポロジカルソート順
    for v in 0..n {
        if seen[v] {
            continue; // すでに訪問済みなら探索しない
        }
        rec(&g, v, &mut seen, &mut order);
    }
    order.reverse(); // 逆順に

    // 出力
    for v in order {
        print!("{} -> ", v);
    }
    println!();
}

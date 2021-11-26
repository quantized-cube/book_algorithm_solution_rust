use proconio::input;

// Union-Find
struct UnionFind {
    par: Vec<i64>,
    siz: Vec<u64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            par: vec![-1; n],
            siz: vec![1; n],
        }
    }

    // 根を求める
    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == -1 {
            return x; // x が根の場合は x を返す
        } else {
            self.par[x] = self.root(self.par[x] as usize) as i64;
            return self.par[x] as usize;
        }
    }

    // x と y が同じグループに属するかどうか (根が一致するかどうか)
    fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    // x を含むグループと y を含むグループとを併合する
    fn unite(&mut self, x: usize, y: usize) -> bool {
        // x, y をそれぞれ根まで移動する
        let mut x = self.root(x);
        let mut y = self.root(y);

        // すでに同じグループのときは何もしない
        if x == y {
            return false;
        }

        // union by size (y 側のサイズが小さくなるようにする)
        if self.siz[x] < self.siz[y] {
            // swap(x, y);
            let tmp = y;
            y = x;
            x = tmp;
        }

        // y を x の子とする
        self.par[y] = x as i64;
        self.siz[x] += self.siz[y];
        return true;
    }

    // x を含むグループのサイズ
    fn size(&mut self, x: usize) -> u64 {
        let r = self.root(x);
        return self.siz[r as usize];
    }
}

fn input() -> (usize, u64, Vec<(usize, usize)>) {
    input! {
        // 頂点数と辺数
        n: usize,
        m: u64,
        // グラフ
        a_b_pairs: [(usize, usize); n],
    }
    return (n, m, a_b_pairs);
}

fn main() {
    // 頂点数と辺数
    let (n, _m, a_b_pairs) = input(use_default_values);

    // Union-Find を要素数 n で初期化
    let mut uf = UnionFind::new(n);

    // 各辺に対する処理
    for ab in a_b_pairs {
        uf.unite(ab.0, ab.1); // a を含むグループと b を含むグループを併合する
    }

    // 集計
    let mut res = 0;
    for x in 0..n {
        if uf.root(x) == x {
            res += 1;
        }
    }
    println!("{}", res);
}

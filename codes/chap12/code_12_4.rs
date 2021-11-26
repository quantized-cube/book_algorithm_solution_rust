use proconio::input;

// i 番目の頂点を根とする部分木について，ヒープ条件を満たすようにする
// a のうち 0 番目から N-1 番目までの部分 a[0:N] についてのみ考える
fn heapify(a: &mut Vec<i64>, i: usize, n: usize) {
    let mut child1 = i * 2 + 1; // 左の子供
    if child1 >= n {
        return; // 子供がないときは終了
    }

    // 子供同士を比較
    if child1 + 1 < n && a[child1 + 1] > a[child1] {
        child1 += 1;
    }

    if a[child1] <= a[i] {
        return; // 逆転がなかったら終了
    }
    // swap
    a.swap(i, child1);

    // 再帰的に
    heapify(a, child1, n);
}

// 配列 a をソートする
fn heap_sort(a: &mut Vec<i64>) {
    let n = a.len();

    // ステップ 1: a 全体をヒープにするフェーズ
    for i in (0..n / 2).rev() {
        heapify(a, i, n);
    }

    // ステップ 2: ヒープから 1 個 1 個最大値を pop するフェーズ
    for i in (1..n).rev() {
        a.swap(0, i); // ヒープの最大値を左詰め
        heapify(a, 0, i); // ヒープサイズは i に
    }
}

fn input() -> (usize, Vec<i64>) {
    input! {
        n: usize, // 要素数
        a: [i64; n],
    }
    (n, a)
}

fn main() {
    // 入力
    let (_n, mut a) = input();

    // ヒープソート
    heap_sort(&mut a);
}

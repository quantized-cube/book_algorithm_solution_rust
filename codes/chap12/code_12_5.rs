use proconio::input;

const MAX: usize = 100_000; // ここでは配列の値は 100000 未満とする

// バケットソート
fn bucket_sort(a: &mut Vec<usize>) {
    let n = a.len();

    // 各要素の個数をカウントする
    // num[v]: v の個数
    let mut num = vec![0; MAX];
    for i in 0..n {
        num[a[i]] += 1; // a[i] をカウントする
    }

    // num の累積和をとる
    // sum[v]: v 以下の値の個数
    // 値 a[i] が全体の中で何番目に小さいかを求める
    let mut sum = vec![0; MAX];
    sum[0] = num[0];
    for v in 1..MAX {
        sum[v] = sum[v - 1] + num[v];
    }

    // sum をもとにソート処理
    // a2: a をソートしたもの
    let mut a2 = vec![0; n];
    for i in (0..n).rev() {
        sum[a[i]] -= 1;
        a2[sum[a[i]]] = a[i];
    }
    *a = a2;
}

fn input() -> (usize, Vec<usize>) {
    input! {
        n: usize, // 要素数
        a: [usize; n],
    }
    (n, a)
}

fn main() {
    // 入力
    let (_n, mut a) = input();

    // バケットソート
    bucket_sort(&mut a);
}

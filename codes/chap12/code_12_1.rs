use proconio::input;

// 配列 a をソートする
fn insertion_sort(a: &mut Vec<i64>) {
    let n = a.len();
    for i in 1..n {
        let v = a[i]; // 挿入したい値

        // v を挿入する適切な場所 j を探す
        let mut j = i;
        while j > 0 {
            if a[j - 1] > v {
                // v より大きいものは 1 つ後ろに移す
                a[j] = a[j - 1];
            } else {
                // v 以下になったら止める
                break;
            }
            j -= 1;
        }
        a[j] = v; // 最後に j 番目に v をもってくる
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

    // 挿入ソート
    insertion_sort(&mut a);
}

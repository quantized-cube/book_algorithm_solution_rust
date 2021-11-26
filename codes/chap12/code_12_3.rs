use proconio::input;

// 配列 a の区間 [left, right) をソートする
// [left, right) は，left, left+1, ..., right-1 番目を表す
fn quick_sort(a: &mut Vec<i64>, left: usize, right: usize) {
    if right - left <= 1 {
        return;
    }

    let pivot_index = left + (right - left) / 2; // 適当にここでは中点とします
    let pivot = a[pivot_index];
    (*a).swap(pivot_index, right - 1); // pivot と右端を swap

    let mut i = left; // i は左詰めされた pivot 未満要素の右端を表す
    for j in left..(right - 1) {
        if a[j] < pivot {
            // pivot 未満のものがあったら左に詰めていく
            (*a).swap(i, j);
            i += 1;
        }
    }
    (*a).swap(i, right - 1); // pivot を適切な場所に挿入

    // println!("{:?}", a);
    // println!("{}", i);

    // 再帰的に解く
    quick_sort(a, left, i); // 左半分 (pivot 未満)
    quick_sort(a, i + 1, right); // 右半分 (pivot 以上)
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
    let (n, mut a) = input();

    // クイックソート
    quick_sort(&mut a, 0, n);
}

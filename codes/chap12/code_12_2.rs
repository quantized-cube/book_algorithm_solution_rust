use proconio::input;

// 配列 a の区間 [left, right) をソートする
// [left, right) は，left, left+1, ..., right-1 番目を表す
fn merge_sort(a: &mut Vec<i64>, left: usize, right: usize) {
    if right - left == 1 {
        return;
    }
    let mid = left + (right - left) / 2;

    // 左半分 [left, mid) をソート
    merge_sort(a, left, mid);

    // 右半分 [mid, right) をソート
    merge_sort(a, mid, right);

    // いったん「左」と「右」のソート結果をコピーしておく (右側は左右反転)
    let mut buf = vec![];
    for i in left..mid {
        buf.push(a[i]);
    }
    for i in (mid..right).rev() {
        buf.push(a[i]);
    }

    // 併合する
    let mut index_left = 0; // 左側の添字
    let mut index_right = buf.len() - 1; // 右側の添字

    for i in left..right {
        if buf[index_left] <= buf[index_right] {
            // 左側採用
            a[i] = buf[index_left];
            index_left += 1;
        } else {
            // 右側採用
            a[i] = buf[index_right];
            index_right -= 1;
        }
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
    let (n, mut a) = input();

    // マージソート
    merge_sort(&mut a, 0, n);
}

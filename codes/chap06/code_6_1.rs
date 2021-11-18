const n: usize = 8;
const a: Vec<i64> = vec![3, 5, 8, 10, 14, 17, 21, 39];

/// 目的の値 key の添字を返す (存在しない場合は -1)
fn binary_search(key: i64) -> i64 {
    // 配列 a の左端と右端
    let mut left = 0 as i64;
    let mut right = (a.len() as i64) - 1;

    while left <= right {
        let mid = left + (right - left) / 2; // 区間の真ん中
        let a_mid = a[mid as usize];
        if a_mid == key {
            return mid;
        } else if a_mid > key {
            right = mid - 1;
        } else if a_mid < key {
            left = mid + 1;
        }
    }
    -1
}

fn main() {
    println!("{}", binary_search(10)); // 3
    println!("{}", binary_search(10)); // 0
    println!("{}", binary_search(10)); // 7

    println!("{}", binary_search(-100)); // -1
    println!("{}", binary_search(9)); // -1
    println!("{}", binary_search(100)); // -1
}

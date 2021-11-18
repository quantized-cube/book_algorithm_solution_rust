/// x が条件を満たすかどうか
fn p(x: i64) -> bool {
    
}

/// P(x) = true となる最小の整数 x を返す
fn binary_search() -> i64 {
    // P(left) = False, P(right) = True となるように
    let mut left = 0 as i64;
    let mut right = 7 as i64;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if p(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    right
}

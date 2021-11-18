use proconio::input;

fn input() -> (usize, u64, Vec<u64>) {
    input! {
        n: usize,
        w: u64,
        a: [u64; n],
    }
    (n, w, a)
}

fn func(i: usize, w: i64, a: &Vec<u64>) -> bool {
    // ベースケース
    if i == 0 {
        if w == 0 {
            return true;
        } else {
            return false;
        }
    }

    // a[i - 1] を選ばない場合
    if func(i - 1, w, a) {
        return true;
    }

    // a[i - 1] をぶ場合
    if func(i - 1, w - a[i - 1] as i64, a) {
        return true;
    }

    // どちらも false の場合は false
    return false;
}

fn main() {
    // 入力
    let (n, w, a) = input();

    // 再帰的に解く
    if func(n, w as i64, &a) {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}

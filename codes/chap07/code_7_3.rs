use proconio::input;

fn input() -> (usize, Vec<u64>, Vec<u64>) {
    input! {
        n: usize,
        a_b_pair: [(u64, u64); n],
    }
    let mut a: Vec<u64> = Vec::new();
    let mut b: Vec<u64> = Vec::new();
    for item in a_b_pair.into_iter() {
        a.push(item.0);
        b.push(item.1);
    }
    return (n, a, b);
}

fn main() {
    // 入力
    let (n, mut a, b) = input();

    // 答え
    let mut sum = 0;
    for i in (0..n).rev() {
        a[i] += sum; // 前回までの操作回数を足す
        let remainder = a[i] % b[i];
        let mut d = 0;
        if remainder != 0 {
            d = b[i] - remainder;
        }
        sum += d;
    }
    println!("{}", sum);
}

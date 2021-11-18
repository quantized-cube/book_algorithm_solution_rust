use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut count: u64 = 0;
    for _ in 0..n {
        for _ in 0..n {
            count += 1;
        }
    }
}

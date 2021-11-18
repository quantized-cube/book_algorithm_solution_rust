use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut i = 2;
    while i <= n {
        println!("{}", i);
        i += 2;
    }
}

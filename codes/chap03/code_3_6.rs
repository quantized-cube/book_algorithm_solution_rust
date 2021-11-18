use proconio::input;

fn input(use_default_values: bool) -> (usize, u64, Vec<u64>) {
    input! {
        n: usize,
        w: u64,
        a: [u64; n],
    }
    (n, w, a)
}

fn main() {
    // 入力受け取り
    let (n, w, a) = input();

    // bit は 2^n 通りの部分集合全体を動く
    let mut exist = false;
    for bit in 0..(1 << n) {
        let mut sum = 0; // 部分集合に含まれる要素の和
        for i in 0..n {
            // i 番目の要素 a[i] が部分集合に含まれているかどうか
            if bit & (1 << i) != 0 {
                sum += a[i];
            }
        }

        // sum が w に一致するかどうか
        if sum == w {
            exist = true;
        }
    }

    if exist {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}

use proconio::input;

// コインの金額
const VALUE: [u64; 6] = [500, 100, 50, 10, 5, 1];

fn input() -> (u64, Vec<u64>) {
    input! {
        x: u64,
        a: [u64; 6],
    }
    return (x, a);
}

fn main() {
    // 入力
    let (mut x, a) = input();

    // 貪欲法
    let mut result = 0;
    for i in 0..6 {
        // 枚数制限がない場合の枚数
        let mut add = x / VALUE[i];

        // 枚数制限を考慮
        if add > a[i] {
            add = a[i];
        }

        // 残り金額を求めて，答えに枚数を加算する
        x -= VALUE[i] * add;
        result += add;
    }
    println!("{}", result);
}

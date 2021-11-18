use proconio::input;

fn main() {
    println!("Start Game!");

    // A さんの数の候補を表す区間を、[left, right) と表す
    let mut left: i64 = 20;
    let mut right: i64 = 36;

    // A さんの数を 1 つに絞れないうちは繰り返す
    while right - left > 1 {
        let mid = left + (right - left) / 2; // 区間の真ん中

        // mid 以上かを聞いて、回答を yes/no で受け取る
        println!("Is the age less than {}? (yes / no)", mid);
        input! {
            ans: String,
        }

        // 回答に応じて、ありうる数の範囲を絞る
        if ans == "yes" {
            right = mid;
        } else {
            left = mid;
        }
    }

    // ズバリ当てる！
    println!("The age is {}!", left);
}

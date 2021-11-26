const MAX: usize = 100_000; // スタック配列の最大サイズ

static mut ST: [i32; MAX] = [0; MAX]; // スタックを表す配列
static mut TOP: usize = 0; // スタックの先頭を表す添字

// スタックを初期化する
fn init() {
    unsafe {
        TOP = 0; // スタックの添字を初期位置に
    }
}

// スタックが空かどうかを判定する
fn is_empty() -> bool {
    unsafe {
        TOP == 0 // スタックサイズが 0 かどうか
    }
}

// スタックが満杯かどうかを判定する
fn is_full() -> bool {
    unsafe {
        TOP == MAX // スタックサイズが MAX かどうか
    }
}

// push
fn push(x: i32) {
    if is_full() {
        println!("error: stack is full.");
        return;
    }
    unsafe {
        ST[TOP] = x; // x を格納して
        TOP += 1; // TOP を進める
    }
}

// pop
fn pop() -> i32 {
    if is_empty() {
        println!("error: stack is empty.");
        return -1;
    }
    unsafe {
        TOP -= 1; // TOP をデクリメントして
        return ST[TOP]; // TOP の位置にある要素を返す
    }
}

fn main() {
    init(); // スタックを初期化

    push(3); // スタックに 3 を挿入する {} -> {3}
    push(5); // スタックに 5 を挿入する {3} -> {3, 5}
    push(7); // スタックに 7 を挿入する {3, 5} -> {3, 5, 7}

    println!("{}", pop()); // {3, 5, 7} -> {3, 5} で 7 を出力
    println!("{}", pop()); // {3, 5} -> {3} で 5 を出力

    push(9); // 新たに 9 を挿入する {3} -> {3, 9}
}

const MAX: usize = 100_000; // キュー配列の最大サイズ

static mut QU: [i32; MAX] = [0; MAX]; // キューを表す配列
static mut TAIL: usize = 0; // キューの要素区間を表す変数
static mut HEAD: usize = 0; // キューの要素区間を表す変数

// キューを初期化する
fn init() {
    unsafe {
        TAIL = 0;
        HEAD = 0;
    }
}

// キューが空かどうかを判定する
fn is_empty() -> bool {
    unsafe { HEAD == TAIL }
}

// キューが満杯かどうかを判定する
fn is_full() -> bool {
    unsafe { HEAD == (TAIL + 1) % MAX }
}

// enqueue
fn enqueue(x: i32) {
    if is_full() {
        println!("error: queue is full.");
        return;
    }
    unsafe {
        QU[TAIL] = x;
        TAIL += 1;
        if TAIL == MAX {
            TAIL = 0;
        } // リングバッファの終端に来たら 0 に
    }
}

// dequeue
fn dequeue() -> i32 {
    if is_empty() {
        println!("error: queue is empty.");
        return -1;
    }
    unsafe {
        let res = QU[HEAD];
        HEAD += 1;
        if HEAD == MAX {
            HEAD = 0;
        } // リングバッファの終端に来たら 0 に
        res
    }
}

fn main() {
    init(); // キューを初期化

    enqueue(3); // キューに 3 を挿入する {} -> {3}
    enqueue(5); // キューに 5 を挿入する {3} -> {3, 5}
    enqueue(7); // キューに 7 を挿入する {3, 5} -> {3, 5, 7}

    println!("{}", dequeue()); // {3, 5, 7} -> {5, 7} で 3 を出力
    println!("{}", dequeue()); // {5, 7} -> {7} で 5 を出力

    enqueue(9); // 新たに 9 を挿入する {7} -> {7, 9}
}

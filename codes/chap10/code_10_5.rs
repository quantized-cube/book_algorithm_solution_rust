struct Heap {
    heap: Vec<i64>,
}

impl Heap {
    fn new() -> Self {
        Self { heap: vec![] }
    }

    // ヒープに値 x を挿入
    fn push(&mut self, x: i64) {
        self.heap.push(x); // 最後尾に挿入
        let mut i = self.heap.len() - 1; // 挿入された頂点番号
        while i > 0 {
            let p = (i - 1) / 2; // 親の頂点番号
            if self.heap[p] >= x {
                break;
            } // 逆転がなければ終了
            self.heap[i] = self.heap[p]; // 自分の値を親の値にする
            i = p; // 自分は上に行く
        }
        self.heap[i] = x; // x は最終的にはこの位置にもってくる
    }

    // 最大値を知る
    fn top(&self) -> i64 {
        if !self.heap.is_empty() {
            return self.heap[0];
        } else {
            return -1;
        }
    }

    // 最大値を削除
    fn pop(&mut self) {
        if self.heap.is_empty() {
            return;
        }
        let x = self.heap.pop().unwrap(); // 頂点にもってくる値
        let mut i = 0; // 根から降ろしていく
        while i * 2 + 1 < self.heap.len() {
            // 子頂点同士を比較して大きい方を child1 とする
            let mut child1 = i * 2 + 1;
            let child2 = i * 2 + 2;
            if child2 < self.heap.len() && self.heap[child2] > self.heap[child1] {
                child1 = child2;
            }
            if self.heap[child1] <= x {
                break;
            } // 逆転がなければ終了
            self.heap[i] = self.heap[child1]; // 自分の値を子頂点の値にする
            i = child1; // 自分は下に行く
        }
        self.heap[i] = x; // x は最終的にはこの位置にもってくる
    }
}

fn main() {
    let mut h = Heap::new();
    h.push(5);
    h.push(3);
    h.push(7);
    h.push(1);

    println!("{}", h.top()); // 7
    h.pop();
    println!("{}", h.top()); // 5

    h.push(11);
    println!("{}", h.top()); // 11
}

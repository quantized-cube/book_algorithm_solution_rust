fn main() {
    let mut f: Vec<u64> = vec![0; 50];
    f[1] = 1;
    for n in 2..50 {
        f[n] = f[n - 1] + f[n - 2];
        println!("{} 項目 = {}", n, f[n]);
    }
}

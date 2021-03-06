fn root(x: usize) -> usize {
    if (par[x] == -1) {
        return x; // x が根の場合は x を直接返す
    } else {
        par[x] = root(par[x]); // x の親 par[x] を根に設定する
        return par[x];
    }
}

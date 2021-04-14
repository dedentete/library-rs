#![allow(non_snake_case, unused)]
use proconio::{marker::*, *};
use std::{cmp::*, collections::*, *};
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.root(self.par[x]);
            self.par[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y {
            return;
        }
        if self.siz[x] < self.siz[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.siz[x] += self.siz[y];
        self.par[y] = x;
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}

fn main() {
    input! {
        n : usize,
        q : usize,
        pab : [(usize, usize, usize); q],
    }
    let mut uf = UnionFind::new(n);
    for (p, a, b) in pab {
        if p == 0 {
            uf.unite(a, b);
        } else {
            if uf.same(a, b) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

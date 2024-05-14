// union find for usize
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect::<Vec<usize>>(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        while x != self.parent[x] {
            x = self.parent[x];
        }
        x
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

        if self.size[x_root] < self.size[y_root] {
            self.parent[x_root] = y_root;
            self.size[y_root] += self.size[x_root];
        } else {
            self.parent[y_root] = x_root;
            self.size[x_root] += self.size[y_root];
        }
    }

    pub fn same_set(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

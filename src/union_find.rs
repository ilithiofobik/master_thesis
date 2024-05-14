// union find for usize
struct UnionFind {
    parent: Vec<Option<usize>>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![None; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while let Some(p) = self.parent[x] {
            x = p;
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

        if self.size[x_root] < self.size[y_root] {
            self.parent[x_root] = Some(y_root);
            self.size[y_root] += self.size[x_root];
        } else {
            self.parent[y_root] = Some(x_root);
            self.size[x_root] += self.size[y_root];
        }
    }

    fn same_set(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

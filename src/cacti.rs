extern crate ndarray;
extern crate ndarray_linalg;

use crate::graphs::Graph;
use crate::triangle_listing::*;
use crate::union_find::*;

pub fn basic_cacti_approximation(g: &Graph) -> Graph {
    let n = g.num_of_vertices();
    let mut result = Graph::empty(n);

    let mut union_find = UnionFind::new(n);
    let triangle_lister = TriangleLister::new(g);

    // get maximal triangular cactus
    for (a, b, c) in triangle_lister {
        let pairs = [(a, b), (b, c), (c, a)];

        if pairs.iter().any(|&(x, y)| union_find.same_set(x, y)) {
            continue;
        }

        for (x, y) in pairs {
            union_find.union(x, y);
            result.add_edge(x, y);
        }
    }

    // extend to maximal structure
    for u in g.vertices() {
        for &v in g.neighbors(u).unwrap() {
            if union_find.same_set(u, v) {
                continue;
            }
            union_find.union(u, v);
            result.add_edge(u, v);
        }
    }

    result
}

use crate::graphs::Graph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::RwLock;
use std::vec;

fn sort_3tuple<T>(a: T, b: T, c: T) -> (T, T, T)
where
    T: PartialOrd,
{
    if a <= b {
        if b <= c {
            (a, b, c)
        } else if a <= c {
            (a, c, b)
        } else {
            (c, a, b)
        }
    } else if a <= c {
        (b, a, c)
    } else if b <= c {
        (b, c, a)
    } else {
        (c, b, a)
    }
}

pub fn list_triangles(g: &Graph) -> Vec<(usize, usize, usize)> {
    let n = g.num_of_vertices();
    let mut triangles = Vec::new();
    let mut marked = HashSet::new();
    let mut processed = vec![false; n];

    let mut sorted_vertices = g.vertices().collect::<Vec<usize>>();
    sorted_vertices.sort_by(|a, b| g.degree(*b).cmp(&g.degree(*a)));

    while let Some(v) = sorted_vertices.pop() {
        let v_neighbours = g.neighbours(v).unwrap();
        for u in v_neighbours {
            if !processed[*u] {
                marked.insert(u);
            }
        }

        while !marked.is_empty() {
            let u = *marked.iter().next().unwrap();
            marked.remove(u);
            let u_neighbours = g.neighbours(*u).unwrap();

            for w in u_neighbours {
                if marked.contains(w) {
                    let triangle = sort_3tuple(v, *u, *w);
                    triangles.push(triangle);
                }
            }
        }

        processed[v] = true;
    }

    triangles
}

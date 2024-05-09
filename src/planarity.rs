use crate::graphs::DirectedGraph;
use crate::graphs::Graph;
use std::collections::HashMap;

struct Planarity<'a> {
    graph: &'a Graph,
    n: usize,
    parent_edge: Vec<Option<(usize, usize)>>,
    orient: HashMap<(usize, usize), (usize, usize)>,
    lowpt: HashMap<(usize, usize), usize>,
    lowpt2: HashMap<(usize, usize), usize>,
    nesting_depth: HashMap<(usize, usize), usize>,
    height: Vec<Option<usize>>,
}

impl Planarity<'_> {
    pub fn new(graph: &Graph) -> Planarity {
        let n = graph.num_of_vertices();
        let m = graph.num_of_edges();
        let height = vec![None; n];
        let parent_edge = vec![None; n];
        let orient = HashMap::with_capacity(2 * m);
        let lowpt = HashMap::with_capacity(m);
        let lowpt2 = HashMap::with_capacity(m);
        let nesting_depth = HashMap::with_capacity(m);

        Planarity {
            graph,
            n,
            parent_edge,
            orient,
            lowpt,
            lowpt2,
            nesting_depth,
            height,
        }
    }

    fn dfs1(&mut self, v: usize) {
        let e = self.parent_edge[v];

        for &w in self.graph.neighbors(v).unwrap() {
            if !self.orient.contains_key(&(v, w)) {
                // set orientation
                self.orient.insert((v, w), (v, w));
                self.orient.insert((w, v), (v, w));
                // set lowpoints
                self.lowpt.insert((v, w), self.height[v].unwrap());
                self.lowpt2.insert((v, w), self.height[v].unwrap());

                // if tree edge set parent and height and go deeper
                // else set lowpt to be the height of w
                if self.height[w].is_none() {
                    self.parent_edge[w] = Some((v, w));
                    self.height[w] = Some(self.height[v].unwrap() + 1);
                    self.dfs1(w);
                } else {
                    self.lowpt.insert((v, w), self.height[w].unwrap());
                }

                // determine nesting depth
                self.nesting_depth.insert((v, w), 2 * self.lowpt[&(v, w)]);
                if self.lowpt2[&(v, w)] < self.height[v].unwrap() {
                    self.nesting_depth
                        .insert((v, w), self.nesting_depth[&(v, w)] + 1);
                }

                // update lowpoints of parent edge
                if let Some(e) = e {
                    let lowptvw = self.lowpt[&(v, w)];
                    let lowpte = self.lowpt[&e];
                    let lowpt2vw = self.lowpt2[&(v, w)];
                    let lowpt2e = self.lowpt2[&e];

                    if lowptvw < lowpte {
                        self.lowpt2.insert(e, std::cmp::min(lowpte, lowpt2vw));
                        self.lowpt.insert(e, lowptvw);
                    } else if lowptvw > lowpte {
                        self.lowpt2.insert(e, std::cmp::min(lowpt2e, lowptvw));
                    } else {
                        self.lowpt2.insert(e, std::cmp::min(lowpt2e, lowpt2vw));
                    }
                }
            }
        }
    }

    // runs a planar test for simple connected graphs
    pub fn is_planar(&mut self) -> bool {
        // Orientation phase
        self.height[0] = Some(0);
        self.dfs1(0);

        // Testing phase

        true
    }
}

fn trivial_test(graph: &Graph) -> Option<bool> {
    let n = graph.num_of_vertices();
    let m = graph.num_of_edges();

    if n <= 4 {
        return Some(true);
    }

    if m > 3 * n - 6 {
        return Some(false);
    }

    None
}

fn is_planar_connected(graph: &Graph) -> bool {
    if let Some(result) = trivial_test(graph) {
        return result;
    }

    let mut planarity = Planarity::new(graph);
    planarity.is_planar()
}

pub fn is_planar(graph: &Graph) -> bool {
    if let Some(result) = trivial_test(graph) {
        return result;
    }

    let connected_components = split_graph_into_connected(graph);
    for component in connected_components {
        if !is_planar_connected(&component) {
            return false;
        }
    }
    true
}

pub fn split_graph_into_connected(graph: &Graph) -> Vec<Graph> {
    let n = graph.num_of_vertices();
    let mut connected_components = vec![];
    let mut queue = Vec::new();
    let mut new_index = vec![None; n];

    for v in graph.vertices() {
        if new_index[v].is_none() {
            let mut component = Graph::new();
            new_index[v] = Some(component.add_vertex());
            queue.push(v);

            while let Some(u) = queue.pop() {
                for w in graph.neighbors(u).unwrap() {
                    if new_index[*w].is_none() {
                        new_index[*w] = Some(component.add_vertex());
                        queue.push(*w);
                    }

                    component.add_edge(new_index[u].unwrap(), new_index[*w].unwrap());
                }
            }

            connected_components.push(component);
        }
    }

    connected_components
}

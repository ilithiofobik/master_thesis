use crate::graphs::Graph;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Default)]
struct Interval {
    low: Option<(usize, usize)>,
    high: Option<(usize, usize)>,
}

#[derive(Clone, Copy, PartialEq, Default)]
struct ConflictPair {
    left: Interval,
    right: Interval,
}

impl Interval {
    pub fn is_none(&self) -> bool {
        self.low.is_none() && self.high.is_none()
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}

impl ConflictPair {
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
    }

    pub fn is_none(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}

struct Planarity<'a> {
    graph: &'a Graph,
    n: usize,
    parent_edge: Vec<Option<(usize, usize)>>,
    orient: HashMap<(usize, usize), (usize, usize)>,
    lowpt: HashMap<(usize, usize), usize>,
    lowpt2: HashMap<(usize, usize), usize>,
    nesting_depth: HashMap<(usize, usize), usize>,
    height: Vec<Option<usize>>,
    s: Vec<ConflictPair>,
    stack_bottom: HashMap<(usize, usize), ConflictPair>,
    lowpt_edge: HashMap<(usize, usize), (usize, usize)>,
    reff: HashMap<(usize, usize), (usize, usize)>,
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
        let s = Vec::new();
        let stack_bottom = HashMap::with_capacity(m);
        let lowpt_edge = HashMap::with_capacity(m);
        let reff = HashMap::with_capacity(m);

        Planarity {
            graph,
            n,
            parent_edge,
            orient,
            lowpt,
            lowpt2,
            nesting_depth,
            height,
            s,
            stack_bottom,
            lowpt_edge,
            reff,
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

                    match lowptvw.cmp(&lowpte) {
                        std::cmp::Ordering::Less => {
                            self.lowpt2.insert(e, std::cmp::min(lowpte, lowpt2vw));
                            self.lowpt.insert(e, lowptvw);
                        }
                        std::cmp::Ordering::Greater => {
                            self.lowpt2.insert(e, std::cmp::min(lowpt2e, lowptvw));
                        }
                        std::cmp::Ordering::Equal => {
                            self.lowpt2.insert(e, std::cmp::min(lowpt2e, lowpt2vw));
                        }
                    }
                }
            }
        }
    }

    fn is_conflicting(&self, i: Interval, b: (usize, usize)) -> bool {
        i.is_some() && self.lowpt[&i.high.unwrap()] > self.lowpt[&b]
    }

    fn add_edge_constraints(&mut self, e_i: (usize, usize), e: (usize, usize)) -> bool {
        let mut p = ConflictPair::default();

        // merge return edges of e_i into P.right
        loop {
            if let Some(mut q) = self.s.pop() {
                if q.left.is_some() {
                    q.swap();
                }

                if q.left.is_some() {
                    return false;
                }

                if self.lowpt[&q.right.low.unwrap()] > self.lowpt[&e] {
                    if p.right.is_none() {
                        p.right.high = q.right.high;
                    } else {
                        self.reff.insert(p.right.low.unwrap(), q.right.low.unwrap());
                    }

                    p.right.low = q.right.low;
                } else {
                    self.reff.insert(q.right.low.unwrap(), self.lowpt_edge[&e]);
                }
            }

            if Some(&self.stack_bottom[&e_i]) == self.s.last() {
                break;
            }
        }

        // merge conflicting return edges of e_1,..,e_{i-1} into P.L
        loop {
            if self.s.is_empty() {
                break;
            }

            let top = self.s.last().unwrap();

            if !self.is_conflicting(top.left, e_i) && !self.is_conflicting(top.right, e_i) {
                break;
            }

            let mut q = self.s.pop().unwrap();
            if self.is_conflicting(q.right, e_i) {
                q.swap();
            }
            if self.is_conflicting(q.right, e_i) {
                return false;
            }

            // merge interval below lowpt(e_i) into p.r
            self.reff
                .insert(p.right.low.unwrap(), q.right.high.unwrap());
            if let Some(qrlow) = q.right.low {
                p.right.low = Some(qrlow);
            }

            if p.left.is_none() {
                p.left.high = q.left.high;
            } else {
                self.reff.insert(p.left.low.unwrap(), q.left.high.unwrap());
            }

            p.left.low = q.left.low;
        }

        if p != ConflictPair::default() {
            self.s.push(p);
        }

        true
    }

    fn lowest(&self, p: &ConflictPair) -> usize {
        if p.left.is_none() {
            return self.lowpt[&p.right.low.unwrap()];
        }

        if p.right.is_none() {
            return self.lowpt[&p.left.low.unwrap()];
        }

        return std::cmp::min(
            self.lowpt[&p.left.low.unwrap()],
            self.lowpt[&p.right.low.unwrap()],
        );
    }

    fn trim_backedges_ending_at_parent(&mut self, u: usize) {
        while !self.s.is_empty() && self.lowest(self.s.last().unwrap()) == self.height[u].unwrap() {
            self.s.pop();
        }

        if !self.s.is_empty() {
            let mut p = self.s.pop().unwrap();

            while p.left.high.is_some() && p.left.high.unwrap().0 == u {
                p.left.high = Some(self.reff[&p.left.high.unwrap()]);
            }

            if p.left.high.is_none() && p.left.low.is_some() {
                self.reff.insert(p.left.low.unwrap(), p.right.low.unwrap());
                p.left.low = None;
            }

            self.s.push(p);
        }
    }

    fn dfs2(&mut self, v: usize) -> bool {
        let e = self.parent_edge[v];
        // get outgoing edges and sort them by nesting_depth
        let mut outgoing_edges = self
            .graph
            .neighbors(v)
            .unwrap()
            .iter()
            .map(|&w| (v, w))
            .filter(|&e| self.orient[&e] == e)
            .collect::<Vec<_>>();
        outgoing_edges.sort_by(|&a, &b| self.nesting_depth[&a].cmp(&self.nesting_depth[&b]));

        for (i, &e_i) in outgoing_edges.iter().enumerate() {
            if let Some(top) = self.s.last() {
                self.stack_bottom.insert(e_i, *top);
            }

            // if tree edge go deeper
            // else set lowpt_edge and push conflict pair to stack
            if Some(e_i) == self.parent_edge[e_i.1] {
                self.dfs2(e_i.1);
            } else {
                self.lowpt_edge.insert(e_i, e_i);
                let conflict_pair = ConflictPair {
                    left: Interval::default(),
                    right: Interval {
                        low: Some(e_i),
                        high: Some(e_i),
                    },
                };
                self.s.push(conflict_pair);
            }

            // integrate new return edges
            if self.lowpt[&e_i] < self.height[v].unwrap() {
                if i == 0 {
                    if let Some(e) = e {
                        self.lowpt_edge.insert(e, self.lowpt_edge[&e_i]);
                    }
                } else {
                    if !self.add_edge_constraints(e_i, e.unwrap()) {
                        return false;
                    }
                }
            }

            // remove back edges returning to parent
            if let Some(e) = e {
                let u = e.0;
                self.trim_backedges_ending_at_parent(u);

                if self.lowpt[&e] < self.height[u].unwrap() {
                    if let Some(top) = self.s.last() {
                        let h_l = top.left.high;
                        let h_r = top.right.high;

                        if h_l.is_some()
                            && (h_r.is_none()
                                || self.lowpt[&h_l.unwrap()] > self.lowpt[&h_r.unwrap()])
                        {
                            self.reff.insert(e, self.lowpt_edge[&h_l.unwrap()]);
                        } else {
                            self.reff.insert(e, self.lowpt_edge[&h_r.unwrap()]);
                        }
                    }
                }
            }
        }

        true
    }

    // runs a planar test for simple connected graphs
    pub fn is_planar(&mut self) -> bool {
        // Orientation phase
        self.height[0] = Some(0);
        self.dfs1(0);
        // Testing phase
        self.dfs2(0)
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

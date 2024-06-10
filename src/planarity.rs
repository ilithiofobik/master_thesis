use crate::graphs::Graph;
use rustworkx_core::petgraph;

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

pub fn is_planar(graph: &Graph) -> bool {
    if let Some(result) = trivial_test(graph) {
        println!("Trivial test is enough");
        return result;
    }

    println!("Running full planarity test");
    let edges = graph
        .all_edges()
        .iter()
        .map(|&(u, v)| (u as u32, v as u32))
        .collect::<Vec<_>>();
    let petgraph = petgraph::graph::UnGraph::<usize, ()>::from_edges(&edges);
    rustworkx_core::planar::is_planar(&petgraph)
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

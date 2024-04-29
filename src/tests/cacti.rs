use crate::cacti::list_triangles;
use crate::graphs::Graph;

#[test]
fn list_triangles_test() {
    let graph = Graph::complete(5);
    let mut triangles = list_triangles(&graph);
    triangles.sort();

    assert_eq!(triangles.len(), 10);
    assert_eq!(triangles[0], (0, 1, 2));
    assert_eq!(triangles[1], (0, 1, 3));
    assert_eq!(triangles[2], (0, 1, 4));
    assert_eq!(triangles[3], (0, 2, 3));
    assert_eq!(triangles[4], (0, 2, 4));
    assert_eq!(triangles[5], (0, 3, 4));
    assert_eq!(triangles[6], (1, 2, 3));
    assert_eq!(triangles[7], (1, 2, 4));
    assert_eq!(triangles[8], (1, 3, 4));
    assert_eq!(triangles[9], (2, 3, 4));
}

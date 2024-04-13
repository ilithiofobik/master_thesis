enum PqNodeType {
    P,
    Q,
}

struct PqNode {
    children: Vec<PqNode>,
    node_type: char,
    node_label: char,
}

struct PqTree {
    root: PqNode,
}

impl PqTree {
    pub fn new() -> Self {
        PqTree {
            root: PqNode {
                children: Vec::new(),
                node_type: 'P',
                node_label: '0',
            },
        }
    }
}

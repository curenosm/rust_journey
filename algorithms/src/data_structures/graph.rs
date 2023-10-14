use std::fmt;

pub struct Node {
    pub id: usize,
    pub edges: Vec<usize>,
}

pub struct Edge {
    pub src: usize,
    pub dst: usize,
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node {
            id,
            edges: Vec::new(),
        });
        id
    }

    pub fn add_edge(&mut self, src: usize, dst: usize) {
        self.nodes[src].edges.push(dst);
        self.edges.push(Edge { src, dst });
    }

    pub fn get_node(&self, id: usize) -> Option<&Node> {
        self.nodes.get(id)
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for node in &self.nodes {
            write!(f, "{}: ", node.id)?;
            for edge in &node.edges {
                write!(f, "{} ", edge)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

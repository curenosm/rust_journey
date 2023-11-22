use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub id: usize,
}

#[derive(Debug, PartialEq)]
pub struct Edge {
    pub src: usize,
    pub dst: usize,
}

#[derive(Debug, PartialEq)]
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
        self.nodes.push(Node { id });
        id
    }

    pub fn add_edge(&mut self, src: usize, dst: usize) {
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
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let mut graph = Graph::new();
    }

    #[test]
    fn test_adding_vertex() {
        // Creamos una gráfica y probamos que podemos agregar vertices
        let mut graph = Graph::new();
        let node_1 = graph.add_node();
        let node_2 = graph.add_node();

        assert_eq!(node_1, 0);
        assert_eq!(node_2, 1);
    }

    #[test]
    fn test_adding_edge() {
        // Creamos una gráfica con dos vertices y unimos el primero con el segundo
        let mut graph = Graph::new();
        let node_1 = graph.add_node();
        let node_2 = graph.add_node();
        graph.add_edge(node_1, node_2);

        assert_eq!(graph.edges[0].src, node_1);
        assert_eq!(graph.edges[0].dst, node_2);
    }
}

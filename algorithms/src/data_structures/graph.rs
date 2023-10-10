struct Node {
  id: usize,
  edges: Vec<usize>,
}

struct Edge {
  src: usize,
  dst: usize,
}

struct Graph {
  nodes: Vec<Node>,
  edges: Vec<Edge>,
}

impl Graph {
  fn new() -> Self {
    Graph {
      nodes: Vec::new(),
      edges: Vec::new(),
    }
  }

  fn add_node(&mut self) -> usize {
    let id = self.nodes.len();
    self.nodes.push(Node {
      id,
      edges: Vec::new(),
    });
    id
  }

  fn add_edge(&mut self, src: usize, dst: usize) {
    self.nodes[src].edges.push(dst);
    self.edges.push(Edge { src, dst });
  }

  fn get_node(&self, id: usize) -> Option<&Node> {
    self.nodes.get(id)
  }

  fn get_edge(&self, src: usize, dst: usize) -> Option<&(usize, usize)> {
    self.edges.iter().find(|(s, d)| *s == src && *d == dst)
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

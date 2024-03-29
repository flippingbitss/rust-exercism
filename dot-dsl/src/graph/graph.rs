
use super::graph_items::node::Node;
use super::graph_items::edge::Edge;

use std::collections::HashMap;

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph{
    pub fn new() -> Self {
        Graph { nodes: Vec::new(), edges: Vec::new(), attrs: HashMap::new() }
    }


    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.to_vec();
        self
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = attrs.iter().map(|x| (x.0.into(), x.1.into())).collect();
        self
    }

    pub fn get_attr(&self, id: &str) -> Option<&str> {
       self.attrs.get(id).map(String::as_str)
    }

    pub fn get_node(&self, id: &str) -> Option<Node> {
       self.nodes.iter().find(|&n| n.id == id).cloned()
    }
}
/*
    graph
    This problem requires you to implement a basic graph function
*/

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        // 默认实现（可以被覆盖）
        true
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // 默认实现（可以被覆盖）
    }

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&str> {
        self.adjacency_table().keys().map(|s| s.as_str()).collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_node(&mut self, node: &str) -> bool {
        if self.adjacency_table.contains_key(node) {
            false // 节点已存在
        } else {
            self.adjacency_table.insert(node.to_string(), Vec::new());
            true
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;

        // 确保两个节点都存在于图中
        self.add_node(from);
        self.add_node(to);

        // 无向图需要在两个方向上添加边
        if let Some(neighbors) = self.adjacency_table.get_mut(from) {
            neighbors.push((to.to_string(), weight));
        }

        if let Some(neighbors) = self.adjacency_table.get_mut(to) {
            neighbors.push((from.to_string(), weight));
        }
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        // 提前创建 String 变量
        let a = "a".to_string();
        let b = "b".to_string();
        let c = "c".to_string();

        let expected_edges = [
            (&a, &b, 5),
            (&b, &a, 5),
            (&b, &c, 10),
            (&c, &b, 10),
            (&c, &a, 7),
            (&a, &c, 7),
        ];

        for edge in expected_edges.iter() {
            let (from, to, weight) = edge;
            // 修正比较，确保 `w` 被正确解引用
            assert!(graph
                .edges()
                .iter()
                .any(|(f, t, w)| f == from && t == to && *w == *weight));
        }
    }

    #[test]
    fn test_add_duplicate_node() {
        let mut graph = UndirectedGraph::new();
        assert!(graph.add_node("a"));
        assert!(!graph.add_node("a")); // 重复节点应返回 false
    }

    #[test]
    fn test_contains() {
        let mut graph = UndirectedGraph::new();
        graph.add_node("x");
        assert!(graph.contains("x"));
        assert!(!graph.contains("y"));
    }

    #[test]
    fn test_nodes() {
        let mut graph = UndirectedGraph::new();
        graph.add_node("node1");
        graph.add_node("node2");
        graph.add_node("node3");
        let nodes = graph.nodes();

        // 直接检查节点存在性
        assert!(nodes.contains("node1"));
        assert!(nodes.contains("node2"));
        assert!(nodes.contains("node3"));
    }

    #[test]
    fn test_edges() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 1));
        graph.add_edge(("a", "c", 2));
        graph.add_edge(("b", "c", 3));

        // 提前创建 String 变量
        let a = "a".to_string();
        let b = "b".to_string();
        let c = "c".to_string();

        let expected_edges = vec![
            (&a, &b, 1),
            (&a, &c, 2),
            (&b, &a, 1),
            (&b, &c, 3),
            (&c, &a, 2),
            (&c, &b, 3),
        ];

        for edge in expected_edges.iter() {
            let (from, to, weight) = edge;
            // 修正比较，确保 `w` 被正确解引用
            assert!(graph
                .edges()
                .iter()
                .any(|(f, t, w)| f == from && t == to && *w == *weight));
        }
    }
}

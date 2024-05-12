/*
    graph
    This problem requires you to implement a basic graph functio
*/


use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

/*impl fmt::display for NodeNotInGraph {
    方法的作用是显示一个错误信息。
*/
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}



// 定义一个无向图的数据结构。
// 其中，adjacency_table是一个哈希表，用于存储图的邻接表。
// 每个键值对表示一个节点，其对应的值是一个由该节点连接的节点和权重组成的元组列表。
// 例如，如果图中有两个节点A和B，并且A和B之间的权重为3，那么adjacency_table中应该包含键值对(A, [(B, 3)])。
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

//导入Graph trait。
// 该trait定义了图的基本操作，例如添加节点、添加边等。
// newfunction用于创建一个新的无向图实例。
// adjacency_table_mutable用于获取或设置adjacency_table的引用。
// adjacency_table用于获取adjacency_table的引用。
// add_edge用于添加一条边到图中。
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
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        let (from_node, to_node, weight) = edge;
        let from_node_neighbours = self.adjacency_table_mutable().entry(from_node.to_string()).or_insert(Vec::new());
        from_node_neighbours.push((to_node.to_string(), weight));
        let to_node_neighbours = self.adjacency_table_mutable().entry(to_node.to_string()).or_insert(Vec::new());
        to_node_neighbours.push((from_node.to_string(), weight));

    }
}
 

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        //TODO
        self.adjacency_table_mutable().insert(node.to_string(), Vec::new()).is_some()

    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        let (from_node, to_node, weight) = edge;
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
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
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}

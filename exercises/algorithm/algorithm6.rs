/*
    dfs
    This problem requires you to implement a basic DFS traversal
*/


use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>,
}

/*定义图的邻接表，adj[i]表示节点i的邻接节点列表
 * 每个节点用一个整数表示，范围从0到n-1
 * 每个节点最多连接到n个其他节点
 * 图是无向的，这意味着如果节点i连接到节点j，那么节点j也连接到节点i
 * 图不包含循环，这意味着如果节点i连接到节点j，则j不会连接回i
 * 图不包含平行边，这意味着如果节点i连接到节点j，则不会再次连接到j
 * 图可能包含自环，这意味着节点i连接到它自己
 * 图可能为空，这意味着没有节点连接
 * 图是连通的，这意味着可以从任何节点访问所有其他节点
 * 图是简单图，这意味着一个节点只能连接到其他节点一次
 * 图的节点数量范围从1到100,000
 * 图的边数量范围从0到100,000
 * 图的节点值范围从0到n-1   
 * what do you long to achieve?
 */

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    /*定义添加边的函数，
     * 该函数将两个节点添加到图的邻接表中，
     * 这意味着将边添加到图中，
     * 这意味着将一个节点连接到另一个节点， 
     * self.adj: 表示图的邻接表，
     * src: 表示源节点，
     * dest: 表示目标节点
     * self.adj[src].push(dest);: 将目标节点添加到源节点的邻接表中
     */
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }


    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        //TODO
        visited.insert(v);
        visit_order.push(v);

        // 遍历当前节点的所有邻接节点
        for &adj_node in &self.adj[v] {
            if !visited.contains(&adj_node) {
                self.dfs_util(adj_node, visited, visit_order);
            }
        }
    }
  

    // Perform a depth-first search on the graph, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new();
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]);
    }
}

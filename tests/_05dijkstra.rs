use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::usize::MAX;

// 定义一个结构体来表示图中的边
#[derive(Clone, Debug)]
struct Edge {
    node: usize,
    weight: usize,
}

// 定义图结构，现在使用带权重的边
struct WeightedGraph {
    adj: Vec<Vec<Edge>>,
}

// 用于优先队列的比较结构体
#[derive(Eq, PartialEq)]
struct State {
    distance: usize,
    position: usize,
}

// 让State可以被比较，实现最小堆
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl WeightedGraph {
    // 创建一个新的带权图
    fn new(n: usize) -> Self {
        WeightedGraph {
            adj: vec![vec![]; n],
        }
    }

    // 添加带权边
    fn add_edge(&mut self, src: usize, dest: usize, weight: usize) {
        self.adj[src].push(Edge { node: dest, weight });
        self.adj[dest].push(Edge { node: src, weight }); // 无向图需要双向添加
    }

    // Dijkstra算法实现
    fn dijkstra(&self, start: usize) -> (Vec<usize>, Vec<Option<usize>>) {
        let mut distances = vec![MAX; self.adj.len()];
        let mut previous = vec![None; self.adj.len()];
        let mut heap = BinaryHeap::new();

        // 起始节点距离为0
        distances[start] = 0;
        heap.push(State {
            distance: 0,
            position: start,
        });

        while let Some(State { distance, position }) = heap.pop() {
            // 如果已经找到更短的路径，跳过
            if distance > distances[position] {
                continue;
            }

            // 遍历所有邻居
            for edge in &self.adj[position] {
                let next_distance = distance + edge.weight;

                // 如果找到更短的路径
                if next_distance < distances[edge.node] {
                    distances[edge.node] = next_distance;
                    previous[edge.node] = Some(position);
                    heap.push(State {
                        distance: next_distance,
                        position: edge.node,
                    });
                }
            }
        }

        (distances, previous)
    }

    // 辅助函数：根据previous数组构建路径
    fn build_path(&self, previous: &[Option<usize>], target: usize) -> Vec<usize> {
        let mut path = Vec::new();
        let mut current = target;

        while let Some(prev) = previous[current] {
            path.push(current);
            current = prev;
        }
        path.push(current); // 添加起点
        path.reverse();
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dijkstra() {
        let mut graph = WeightedGraph::new(5);
        graph.add_edge(0, 1, 4);
        graph.add_edge(0, 2, 1);
        graph.add_edge(1, 2, 2);
        graph.add_edge(1, 3, 5);
        graph.add_edge(2, 3, 1);
        graph.add_edge(2, 4, 3);
        graph.add_edge(3, 4, 1);

        let (distances, previous) = graph.dijkstra(0);

        // 检查距离
        assert_eq!(distances[0], 0);
        assert_eq!(distances[1], 3);
        assert_eq!(distances[2], 1);
        assert_eq!(distances[3], 2);
        assert_eq!(distances[4], 3);

        // 检查路径
        assert_eq!(graph.build_path(&previous, 4), vec![0, 2, 3, 4]);
        assert_eq!(graph.build_path(&previous, 1), vec![0, 2, 1]);
    }

    #[test]
    fn test_disconnected_graph() {
        let mut graph = WeightedGraph::new(4);
        graph.add_edge(0, 1, 1);
        graph.add_edge(2, 3, 1);

        let (distances, _) = graph.dijkstra(0);

        assert_eq!(distances[0], 0);
        assert_eq!(distances[1], 1);
        assert_eq!(distances[2], MAX); // 不可达
        assert_eq!(distances[3], MAX); // 不可达
    }
}
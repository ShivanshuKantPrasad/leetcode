// https://leetcode.com/problems/redundant-connection/description/

struct DSU {
    N: usize,
    size: Vec<usize>,
    representative: Vec<usize>,
}

impl DSU {
    pub fn new(N: usize) -> Self {
        DSU {
            N,
            size: vec![1; N],
            representative: (0..N).collect::<Vec<_>>(),
        }
    }

    pub fn find(self: &mut Self, node: usize) -> usize {
        if self.representative[node] == node {
            return node;
        }

        self.representative[node] = self.find(self.representative[node]);
        return self.representative[node];
    }

    pub fn do_union(self: &mut Self, mut node_one: usize, mut node_two: usize) -> bool {
        node_one = self.find(node_one);
        node_two = self.find(node_two);

        if node_one == node_two {
            false
        } else {
            if self.size[node_one] > self.size[node_two] {
                self.representative[node_two] = node_one;
                self.size[node_one] += self.size[node_two];
            } else {
                self.representative[node_one] = node_two;
                self.size[node_two] += self.size[node_one];
            }
            true
        }
    }
}

pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();
    let mut dsu = DSU::new(n);

    for edge in edges {
        if !dsu.do_union(edge[0] as usize - 1, edge[1] as usize - 1) {
            return edge;
        }
    }

    vec![]
}

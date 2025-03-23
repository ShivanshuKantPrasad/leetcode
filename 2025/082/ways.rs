// https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/description/
struct Solution;
impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj_list = vec![vec![]; n];
        for road in roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            let cost = road[2] as u64;
            adj_list[u].push((v, cost));
            adj_list[v].push((u, cost));
        }
        let mut best_cost = vec![u64::MAX; n];
        let mut ways_count = vec![0u64; n];
        best_cost[0] = 0;
        ways_count[0] = 1;
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(std::cmp::Reverse((0u64, 0)));

        while let Some(std::cmp::Reverse((currentCost, currentNode))) = heap.pop() {
            if currentCost > best_cost[currentNode] {
                continue;
            }

            for (neighbor, cost) in adj_list[currentNode].iter() {
                let newCost = currentCost + cost;
                if newCost < best_cost[*neighbor] {
                    best_cost[*neighbor] = newCost;
                    ways_count[*neighbor] = ways_count[currentNode];
                    heap.push(std::cmp::Reverse((newCost, *neighbor)));
                } else if newCost == best_cost[*neighbor] {
                    ways_count[*neighbor] =
                        (ways_count[*neighbor] + ways_count[currentNode]) % 1000000007;
                }
            }
        }
        ways_count[n - 1] as i32
    }
}

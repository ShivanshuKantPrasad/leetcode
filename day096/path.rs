// https://leetcode.com/problems/second-minimum-time-to-reach-destination/submissions/1335622406/

#[derive(Clone, Copy)]
enum Visited {
    None,
    Once(i32),
    Twice,
}

pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
    let mut adj = vec![Vec::<usize>::new(); n as usize + 1];
    for (i, j) in edges.iter().map(|v| (v[0] as usize, v[1] as usize)) {
        adj[i].push(j);
        adj[j].push(i);
    }
    let mut visited = vec![Visited::None; n as usize + 1];
    visited[1] = Visited::Once(0);
    let mut v = vec![1usize];
    let mut v2 = Vec::<usize>::new();
    let mut t = 0;

    loop {
        if (t / change) & 1 == 1 {
            t = (t / change + 1) * change;
        }
        t += time;
        for i in v.drain(..) {
            for &j in adj[i].iter() {
                match (j == n as usize, visited[j]) {
                    (_, Visited::Twice) => (),
                    (true, Visited::Once(t1)) if t1 != t => return t,
                    (_, Visited::Once(t1)) if t1 == t => (),
                    (_, Visited::Once(_)) => {
                        visited[j] = Visited::Twice;
                        v2.push(j);
                    }
                    (_, Visited::None) => {
                        visited[j] = Visited::Once(t);
                        v2.push(j);
                    }
                }
            }
        }
        std::mem::swap(&mut v, &mut v2);
    }
    unreachable!()
}

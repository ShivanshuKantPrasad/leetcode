// https://leetcode.com/problems/regions-cut-by-slashes/description/

pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut dsu = DisjointSetUnion::new(height * (width + width));
    // Each square is divided into two parts: left and right.
    for (row, line) in grid.iter().enumerate() {
        let dsu_offset = row * (width + width);
        // Is each individual cell split?
        for (i, b) in line.bytes().enumerate() {
            let dsu_index_l = dsu_offset + i + i;
            let dsu_index_r = dsu_offset + i + i + 1;
            if b == b' ' {
                dsu.merge(dsu_index_l, dsu_index_r);
            }
        }
        // Neighboring cells are always connected; there are no '|' cells.
        for (i, w) in line.as_bytes().windows(2).enumerate() {
            let dsu_index_l = dsu_offset + i + i + 1;
            let dsu_index_r = dsu_offset + i + i + 2;
            dsu.merge(dsu_index_l, dsu_index_r);
        }
        // Is each cell connected to its upper neighbor?
        if row > 0 {
            let dsu_offset_above = (row - 1) * (width + width);
            for (i, (above, b)) in grid[row - 1].bytes().zip(line.bytes()).enumerate() {
                let dsu_index_a_l = dsu_offset_above + i + i;
                let dsu_index_a_r = dsu_offset_above + i + i + 1;
                let dsu_index_b_l = dsu_offset + i + i;
                let dsu_index_b_r = dsu_offset + i + i + 1;
                match (above, b) {
                    (b' ', b' ') => {
                        dsu.merge(dsu_index_a_l, dsu_index_b_l);
                        dsu.merge(dsu_index_a_r, dsu_index_b_r);
                        dsu.merge(dsu_index_a_l, dsu_index_b_r);
                        dsu.merge(dsu_index_a_r, dsu_index_b_l);
                    }
                    (b' ', b'/') => {
                        dsu.merge(dsu_index_a_l, dsu_index_b_l);
                        dsu.merge(dsu_index_a_r, dsu_index_b_l);
                    }
                    (b' ', b'\\') => {
                        dsu.merge(dsu_index_a_r, dsu_index_b_r);
                        dsu.merge(dsu_index_a_l, dsu_index_b_r);
                    }
                    (b'/', b' ') => {
                        dsu.merge(dsu_index_a_r, dsu_index_b_r);
                        dsu.merge(dsu_index_a_r, dsu_index_b_l);
                    }
                    (b'/', b'/') => {
                        dsu.merge(dsu_index_a_r, dsu_index_b_l);
                    }
                    (b'/', b'\\') => {
                        dsu.merge(dsu_index_a_r, dsu_index_b_r);
                    }
                    (b'\\', b' ') => {
                        dsu.merge(dsu_index_a_l, dsu_index_b_l);
                        dsu.merge(dsu_index_a_l, dsu_index_b_r);
                    }
                    (b'\\', b'/') => {
                        dsu.merge(dsu_index_a_l, dsu_index_b_l);
                    }
                    (b'\\', b'\\') => {
                        dsu.merge(dsu_index_a_l, dsu_index_b_r);
                    }
                    _ => unreachable!("Problem constraints"),
                }
            }
        }
        // println!("{}", dsu.nsets);
    }
    dsu.nsets as i32
}

//region Union-find structure (size-based)
struct DisjointSetUnion {
    roots: Vec<usize>,
    sizes: Vec<usize>,
    nsets: usize,
}

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        Self {
            roots: (0..n).collect(),
            sizes: vec![1; n],
            nsets: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut r = x;
        while self.roots[r] != r {
            r = self.roots[r];
        }
        let r = r;
        let mut x = x;
        while self.roots[x] != r {
            let tmp = self.roots[x];
            self.roots[x] = r;
            x = tmp;
        }
        r
    }

    fn merge(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return false;
        }
        if self.sizes[a] < self.sizes[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.roots[b] = a;
        self.sizes[a] += self.sizes[b];
        self.sizes[b] = 0;
        self.nsets -= 1;
        true
    }
}

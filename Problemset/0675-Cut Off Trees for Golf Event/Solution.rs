use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let m = forest.len();
        let n = forest[0].len();
        let mut trees = vec![];
        let mut seen = HashSet::new();
        let mut cells = VecDeque::new();
        let mut source = (0, 0);
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                if forest[i][j] > 1 {
                    trees.push((i, j));
                }
            }
        }
        trees.sort_unstable_by_key(|&(i, j)| forest[i][j]);

        for &target in &trees {
            seen.clear();
            cells.clear();
            seen.insert(source);
            cells.push_back((source, 0));

            while let Some(((i, j), s)) = cells.pop_front() {
                source = (i, j);

                if source == target {
                    ret += s;
                    break;
                }

                if i > 0 && forest[i - 1][j] != 0 && !seen.contains(&(i - 1, j)) {
                    seen.insert((i - 1, j));
                    cells.push_back(((i - 1, j), s + 1));
                }
                if j > 0 && forest[i][j - 1] != 0 && !seen.contains(&(i, j - 1)) {
                    seen.insert((i, j - 1));
                    cells.push_back(((i, j - 1), s + 1));
                }
                if i + 1 < m && forest[i + 1][j] != 0 && !seen.contains(&(i + 1, j)) {
                    seen.insert((i + 1, j));
                    cells.push_back(((i + 1, j), s + 1));
                }
                if j + 1 < n && forest[i][j + 1] != 0 && !seen.contains(&(i, j + 1)) {
                    seen.insert((i, j + 1));
                    cells.push_back(((i, j + 1), s + 1));
                }
            }

            if source != target {
                return -1;
            }
        }

        ret
    }
}

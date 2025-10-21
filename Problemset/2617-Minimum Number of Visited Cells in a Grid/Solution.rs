use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_visited_cells(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut col_heaps = vec![BinaryHeap::new(); n];
        let mut min_visit = vec![vec![-1; n]; m];
        min_visit[0][0] = 1;

        for i in 0..m {
            let mut row_heap = BinaryHeap::new();

            for j in 0..n {
                while let Some(&(Reverse(visit), k)) = row_heap.peek() {
                    if k < j {
                        row_heap.pop();
                    } else {
                        if min_visit[i][j] == -1 || min_visit[i][j] > visit {
                            min_visit[i][j] = visit;
                        }
                        break;
                    }
                }
                while let Some(&(Reverse(visit), k)) = col_heaps[j].peek() {
                    if k < i {
                        col_heaps[j].pop();
                    } else {
                        if min_visit[i][j] == -1 || min_visit[i][j] > visit {
                            min_visit[i][j] = visit;
                        }
                        break;
                    }
                }

                if min_visit[i][j] != -1 {
                    row_heap.push((Reverse(min_visit[i][j] + 1), grid[i][j] as usize + j));
                    col_heaps[j].push((Reverse(min_visit[i][j] + 1), grid[i][j] as usize + i));
                }
            }
        }

        min_visit[m - 1][n - 1]
    }
}

use std::collections::VecDeque;

impl Solution {
    fn canReach(
        mut grid: Vec<Vec<i32>>,
        mut fire: VecDeque<(usize, usize, i32)>,
        mut minutes: i32,
    ) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut person = VecDeque::from([(0, 0, minutes)]);
        grid[0][0] = -1;

        while let Some(&(i, j, t)) = fire.front() {
            if t >= minutes {
                break;
            }

            if i > 0 && grid[i - 1][j] < 1 {
                grid[i - 1][j] = 1;
                fire.push_back((i - 1, j, t + 1));
            }
            if i + 1 < m && grid[i + 1][j] < 1 {
                grid[i + 1][j] = 1;
                fire.push_back((i + 1, j, t + 1));
            }
            if j > 0 && grid[i][j - 1] < 1 {
                grid[i][j - 1] = 1;
                fire.push_back((i, j - 1, t + 1));
            }
            if j + 1 < n && grid[i][j + 1] < 1 {
                grid[i][j + 1] = 1;
                fire.push_back((i, j + 1, t + 1));
            }

            fire.pop_front();
        }

        while !person.is_empty() {
            while let Some(&(i, j, t)) = person.front() {
                if i == m - 1 && j == n - 1 {
                    return true;
                }

                if t > minutes {
                    break;
                }

                if grid[i][j] < 1 {
                    if i > 0 && grid[i - 1][j] == 0 {
                        grid[i - 1][j] = -1;
                        person.push_back((i - 1, j, t + 1));
                    }
                    if i + 1 < m && grid[i + 1][j] == 0 {
                        grid[i + 1][j] = -1;
                        person.push_back((i + 1, j, t + 1));
                    }
                    if j > 0 && grid[i][j - 1] == 0 {
                        grid[i][j - 1] = -1;
                        person.push_back((i, j - 1, t + 1));
                    }
                    if j + 1 < n && grid[i][j + 1] == 0 {
                        grid[i][j + 1] = -1;
                        person.push_back((i, j + 1, t + 1));
                    }
                }

                person.pop_front();
            }

            while let Some(&(i, j, t)) = fire.front() {
                if t > minutes {
                    break;
                }

                if i > 0 && grid[i - 1][j] < 1 {
                    grid[i - 1][j] = 1;
                    fire.push_back((i - 1, j, t + 1));
                }
                if i + 1 < m && grid[i + 1][j] < 1 {
                    grid[i + 1][j] = 1;
                    fire.push_back((i + 1, j, t + 1));
                }
                if j > 0 && grid[i][j - 1] < 1 {
                    grid[i][j - 1] = 1;
                    fire.push_back((i, j - 1, t + 1));
                }
                if j + 1 < n && grid[i][j + 1] < 1 {
                    grid[i][j + 1] = 1;
                    fire.push_back((i, j + 1, t + 1));
                }

                fire.pop_front();
            }

            minutes += 1;
        }

        false
    }
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut init_fire = VecDeque::new();
        let mut lo = 0;
        let mut hi = (m * n) as i32;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    init_fire.push_back((i, j, 0));
                }
            }
        }

        if !Self::canReach(grid.clone(), init_fire.clone(), lo) {
            return -1;
        }
        if Self::canReach(grid.clone(), init_fire.clone(), hi) {
            return 1_000_000_000;
        }

        while lo < hi {
            let mid = (lo + hi + 1) / 2;

            if Self::canReach(grid.clone(), init_fire.clone(), mid) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        hi
    }
}

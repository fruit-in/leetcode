impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid[0].len();
        let mut answer = (0..n as i32).collect::<Vec<_>>();

        for row in &grid {
            for ball in 0..n {
                if answer[ball] != -1 {
                    if row[answer[ball] as usize] == 1
                        && answer[ball] + 1 < n as i32
                        && row[answer[ball] as usize + 1] == 1
                    {
                        answer[ball] += 1;
                    } else if row[answer[ball] as usize] == -1
                        && answer[ball] > 0
                        && row[answer[ball] as usize - 1] == -1
                    {
                        answer[ball] -= 1;
                    } else {
                        answer[ball] = -1;
                    }
                }
            }
        }

        answer
    }
}

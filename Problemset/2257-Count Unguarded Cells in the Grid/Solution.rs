impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut grid = vec![vec![0; n]; m];
        let mut ret = 0;

        for i in 0..guards.len() {
            grid[guards[i][0] as usize][guards[i][1] as usize] = 1;
        }
        for i in 0..walls.len() {
            grid[walls[i][0] as usize][walls[i][1] as usize] = 2;
        }

        for r in 0..m {
            let mut can_see = false;

            for c in 0..n {
                match grid[r][c] {
                    1 => can_see = true,
                    2 => can_see = false,
                    _ if can_see => grid[r][c] = 3,
                    _ => (),
                }
            }

            can_see = false;

            for c in (0..n).rev() {
                match grid[r][c] {
                    1 => can_see = true,
                    2 => can_see = false,
                    _ if can_see => grid[r][c] = 3,
                    _ => (),
                }
            }
        }

        for c in 0..n {
            let mut can_see = false;

            for r in 0..m {
                match grid[r][c] {
                    1 => can_see = true,
                    2 => can_see = false,
                    _ if can_see => grid[r][c] = 3,
                    _ => (),
                }
            }

            can_see = false;

            for r in (0..m).rev() {
                match grid[r][c] {
                    1 => can_see = true,
                    2 => can_see = false,
                    _ if can_see => grid[r][c] = 3,
                    _ => (),
                }

                if grid[r][c] == 0 {
                    ret += 1;
                }
            }
        }

        ret
    }
}

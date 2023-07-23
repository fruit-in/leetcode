impl Solution {
    pub fn dig_artifacts(n: i32, artifacts: Vec<Vec<i32>>, dig: Vec<Vec<i32>>) -> i32 {
        let mut grid = vec![vec![false; n as usize]; n as usize];
        let mut ret = 0;

        for d in &dig {
            grid[d[0] as usize][d[1] as usize] = true;
        }

        for artifact in &artifacts {
            let mut flag = true;

            for r in artifact[0] as usize..=artifact[2] as usize {
                for c in artifact[1] as usize..=artifact[3] as usize {
                    flag &= grid[r][c];
                }
            }

            ret += flag as i32;
        }

        ret
    }
}

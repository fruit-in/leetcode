impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = land.len();
        let n = land[0].len();
        let mut ret = vec![];

        for r1 in 0..m {
            for c1 in 0..n {
                if land[r1][c1] == 0
                    || (r1 > 0 && land[r1 - 1][c1] == 1)
                    || (c1 > 0 && land[r1][c1 - 1] == 1)
                {
                    continue;
                }

                let mut group = vec![r1 as i32, c1 as i32, r1 as i32, c1 as i32];

                for r2 in r1..m {
                    if land[r2][c1] == 1 {
                        group[2] = r2 as i32;
                    } else {
                        break;
                    }
                }
                for c2 in c1..n {
                    if land[group[2] as usize][c2] == 1 {
                        group[3] = c2 as i32;
                    } else {
                        break;
                    }
                }

                ret.push(group);
            }
        }

        ret
    }
}

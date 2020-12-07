impl Solution {
    pub fn spiral_matrix_iii(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut x = c0;
        let mut y = r0;
        let mut step = 2;
        let direction = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        let mut ret = vec![vec![y, x]];

        while (ret.len() as i32) < r * c {
            for _ in 0..(step / 2) {
                x += direction[step % 4].0;
                y += direction[step % 4].1;

                if x >= 0 && x < c && y >= 0 && y < r {
                    ret.push(vec![y, x]);
                }
            }

            step += 1;
        }

        ret
    }
}

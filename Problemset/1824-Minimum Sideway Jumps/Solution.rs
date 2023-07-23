impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let mut dp0 = [1, 0, 1];

        for &obstacle in &obstacles[1..] {
            let mut dp1 = dp0;

            if obstacle > 0 {
                dp1[obstacle as usize - 1] = i32::MAX;
            }

            let min_sj = *dp1.iter().min().unwrap();

            for i in 0..3 {
                if obstacle as usize != i + 1 {
                    dp1[i] = dp1[i].min(min_sj + 1);
                }
            }

            dp0 = dp1;
        }

        *dp0.iter().min().unwrap()
    }
}

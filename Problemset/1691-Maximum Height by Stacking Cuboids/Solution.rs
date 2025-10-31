impl Solution {
    pub fn max_height(mut cuboids: Vec<Vec<i32>>) -> i32 {
        let mut max_width = 0;
        let mut max_length = 0;

        for cuboid in &mut cuboids {
            cuboid.sort_unstable();
            max_width = max_width.max(cuboid[0]);
            max_length = max_length.max(cuboid[1]);
        }

        cuboids.sort_unstable_by_key(|cuboid| (cuboid[2], cuboid[0], cuboid[1]));

        let mut dp = vec![vec![0; max_length as usize + 1]; max_width as usize + 1];

        for cuboid in &cuboids {
            let (width, length, height) = (cuboid[0] as usize, cuboid[1] as usize, cuboid[2]);

            for i in (0..=width).rev() {
                for j in (0..=length).rev() {
                    dp[width][length] = dp[width][length].max(dp[i][j] + height);
                }
            }
        }

        *dp.iter().flatten().max().unwrap()
    }
}

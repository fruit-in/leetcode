impl Solution {
    pub fn max_building(n: i32, mut restrictions: Vec<Vec<i32>>) -> i32 {
        restrictions.push(vec![1, 0]);
        restrictions.sort_unstable();
        if restrictions.last().unwrap()[0] != n {
            restrictions.push(vec![n, n - 1]);
        }

        for i in 1..restrictions.len() {
            restrictions[i][1] = restrictions[i][1]
                .min(restrictions[i - 1][1] + restrictions[i][0] - restrictions[i - 1][0]);
        }
        for i in (0..restrictions.len() - 1).rev() {
            restrictions[i][1] = restrictions[i][1]
                .min(restrictions[i + 1][1] + restrictions[i + 1][0] - restrictions[i][0]);
        }

        (0..restrictions.len() - 1)
            .map(|i| {
                (restrictions[i + 1][1] + restrictions[i][1] - restrictions[i][0]
                    + restrictions[i + 1][0])
                    / 2
            })
            .max()
            .unwrap()
    }
}

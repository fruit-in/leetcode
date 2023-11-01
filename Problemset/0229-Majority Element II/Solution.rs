impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut majorities = [(i32::MAX, 0); 2];

        for num in &nums {
            if let Some(i) = majorities.iter().position(|(x, _)| x == num) {
                majorities[i].1 += 1;
            } else if let Some(i) = majorities.iter().position(|&(_, c)| c == 0) {
                majorities[i] = (*num, 1);
            } else {
                for i in 0..2 {
                    majorities[i].1 -= 1;
                }
            }
        }

        for i in 0..2 {
            majorities[i].1 = 0;
        }

        for num in &nums {
            if let Some(i) = majorities.iter().position(|(x, _)| x == num) {
                majorities[i].1 += 1;
            }
        }

        majorities
            .into_iter()
            .filter(|&(x, c)| *c > n / 3)
            .map(|(x, _)| *x)
            .collect()
    }
}

impl Solution {
    pub fn make_similar(mut nums: Vec<i32>, mut target: Vec<i32>) -> i64 {
        nums.sort_unstable_by_key(|&x| (x % 2, x));
        target.sort_unstable_by_key(|&x| (x % 2, x));

        nums.iter()
            .zip(target.iter())
            .map(|(&x, &y)| (y - x).max(0) as i64 / 2)
            .sum()
    }
}

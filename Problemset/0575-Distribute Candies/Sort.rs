impl Solution {
    pub fn distribute_candies(candies: Vec<i32>) -> i32 {
        let mut candies = candies;
        candies.sort_unstable();

        let mut sister = 1;

        for i in 1..candies.len() {
            if candies[i] != candies[i - 1] {
                sister += 1;
            }
        }

        sister.min(candies.len() / 2) as i32
    }
}

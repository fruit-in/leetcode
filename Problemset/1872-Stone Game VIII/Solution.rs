impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let mut prefix_sum = stones.iter().sum::<i32>();
        let mut ret = prefix_sum;

        for i in (2..stones.len()).rev() {
            prefix_sum -= stones[i];
            ret = ret.max(prefix_sum - ret);
        }

        ret
    }
}

impl Solution {
    pub fn get_maximum_consecutive(mut coins: Vec<i32>) -> i32 {
        let mut ret = 1;

        coins.sort_unstable();

        for &coin in &coins {
            if coin > ret {
                break;
            }

            ret += coin;
        }

        ret
    }
}

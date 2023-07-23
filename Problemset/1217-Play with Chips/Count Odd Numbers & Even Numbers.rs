impl Solution {
    pub fn min_cost_to_move_chips(chips: Vec<i32>) -> i32 {
        let odd_cnt = chips.iter().filter(|&x| x % 2 == 1).count();
        odd_cnt.min(chips.len() - odd_cnt) as i32
    }
}

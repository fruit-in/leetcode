impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut count = [0_i32; 3];

        for &x in &stones {
            count[x as usize % 3] += 1;
        }

        (count[0] % 2 == 0 && count[1].min(count[2]) > 0)
            || (count[0] % 2 == 1 && (count[2] - count[1]).abs() > 2)
    }
}

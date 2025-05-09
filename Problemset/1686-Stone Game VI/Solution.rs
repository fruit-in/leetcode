impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut indices = (0..bob_values.len()).collect::<Vec<_>>();
        let mut is_bob = false;
        let mut diff = 0;

        indices.sort_unstable_by_key(|&i| -alice_values[i] - bob_values[i]);

        for &i in &indices {
            if is_bob {
                diff -= bob_values[i];
            } else {
                diff += alice_values[i];
            }
            is_bob = !is_bob;
        }

        diff / diff.abs().max(1)
    }
}

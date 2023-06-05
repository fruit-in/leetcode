impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let colors = colors.as_bytes();
        let mut count = 1;
        let mut count_a = 0;
        let mut count_b = 0;

        for i in 1..colors.len() + 1 {
            if i >= colors.len() || colors[i] != colors[i - 1] {
                if colors[i - 1] == b'A' {
                    count_a += (count - 2).max(0);
                } else {
                    count_b += (count - 2).max(0);
                }

                count = 0;
            }

            count += 1;
        }

        count_a > count_b
    }
}

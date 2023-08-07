impl Solution {
    pub fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
        let mut max_points = 0;
        let mut bob_arrows = vec![];

        for x in 1..2_i32.pow(12) {
            let mut points = 0;
            let mut arrows = vec![0; 12];
            let mut arrows_sum = 0;

            for i in 0..12 {
                if x & (1 << i) != 0 {
                    points += i;
                    arrows[i] = alice_arrows[i] + 1;
                    arrows_sum += arrows[i];
                }
            }

            if points > max_points && arrows_sum <= num_arrows {
                max_points = points;
                arrows[0] += num_arrows - arrows_sum;
                bob_arrows = arrows;
            }
        }

        bob_arrows
    }
}

impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut low = 1;
        let mut high = *quantities.iter().max().unwrap();

        while low < high {
            let x = (low + high) / 2;
            let mut y = 0;

            for q in &quantities {
                y += q / x;
                if q % x != 0 {
                    y += 1;
                }
            }

            if y > n {
                low = x + 1;
            } else {
                high = x;
            }
        }

        high
    }
}

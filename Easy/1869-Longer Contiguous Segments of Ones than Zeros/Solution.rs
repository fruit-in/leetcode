impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut max0 = 0;
        let mut max1 = 0;
        let mut count = 0;
        let mut is0 = true;

        for c in s.chars() {
            match (c, is0) {
                ('0', true) | ('1', false) => count += 1,
                (_, true) => {
                    max0 = max0.max(count);
                    is0 = false;
                    count = 1;
                }
                (_, false) => {
                    max1 = max1.max(count);
                    is0 = true;
                    count = 1;
                }
            }
        }

        if is0 {
            max0 = max0.max(count);
        } else {
            max1 = max1.max(count);
        }

        max1 > max0
    }
}

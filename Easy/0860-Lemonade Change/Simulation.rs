impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut five = 0;
        let mut ten = 0;
        for bill in bills {
            if bill == 5 {
                five += 1;
            } else if bill == 10 {
                ten += 1;
                five -= 1;
            } else if ten > 0 {
                ten -= 1;
                five -= 1;
            } else {
                five -= 3;
            }

            if five < 0 {
                return false;
            }
        }

        true
    }
}

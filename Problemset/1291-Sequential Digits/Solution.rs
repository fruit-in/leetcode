impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ret = vec![];

        for i in 1..9 {
            let mut x = i;
            while x <= high && x % 10 != 0 {
                if x >= low {
                    ret.push(x);
                }
                x = x * 10 + x % 10 + 1;
            }
        }

        ret.sort_unstable();

        ret
    }
}

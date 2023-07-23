impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut ret = 0;

        for i in 2..a.len() {
            if a[i] - a[i - 1] == a[i - 1] - a[i - 2] {
                count += 1;
            } else {
                ret += (count - 1) * count / 2;
                count = 1;
            }
        }

        ret + (count - 1) * count / 2
    }
}

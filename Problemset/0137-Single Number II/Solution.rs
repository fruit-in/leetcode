impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bitcount = [0; 32];
        let mut ret = 0;

        for num in &nums {
            for i in 0..32 {
                if (num >> i) & 1 == 1 {
                    bitcount[i] += 1;
                }
            }
        }

        for i in 0..32 {
            if bitcount[i] % 3 == 1 {
                ret |= 1 << i;
            }
        }

        ret
    }
}

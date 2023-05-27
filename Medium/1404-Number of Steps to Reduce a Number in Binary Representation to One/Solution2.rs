impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut carry = false;
        let mut ret = s.len() as i32 - 1;

        for bit in s.chars().rev().take(s.len() - 1) {
            if (bit == '1') ^ carry {
                carry = true;
                ret += 1;
            }
        }

        ret + carry as i32
    }
}

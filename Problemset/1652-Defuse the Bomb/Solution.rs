impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let s = if k > 0 { 1 } else { (n as i32 + k) as usize };
        let e = if k > 0 { k as usize + 1 } else { n };
        let mut sum = code[s..e].iter().sum();
        let mut ret = vec![0; n];

        for i in 0..n {
            ret[i] = sum;
            sum += code[(e + i) % n] - code[(s + i) % n];
        }

        ret
    }
}

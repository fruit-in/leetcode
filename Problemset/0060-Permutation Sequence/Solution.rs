impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut k = k as usize - 1;
        let mut digits = (b'1'..=(n as u8 + b'0')).collect::<Vec<_>>();
        let mut factorials = (0..n).collect::<Vec<_>>();
        let mut ret = vec![b'0'; n];

        factorials[0] = 1;
        for i in 3..n {
            factorials[i] *= factorials[i - 1];
        }

        for i in 0..n {
            ret[i] = digits.remove(k / factorials[n - i - 1]);
            k %= factorials[n - i - 1];
        }

        String::from_utf8(ret).unwrap()
    }
}

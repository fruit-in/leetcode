impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut v = vec![0; n];
 
        let mut i = 1;
        while i * i <= n {
            v[i * i - 1] = 1;
            i += 1;
        }
 
        for i in 2..=n {
            if v[i - 1] == 0 {
                v[i - 1] = i as i32;
                let mut j = 1;
                while j * j < i {
                    v[i - 1] = v[i - 1].min(v[i - j * j - 1] + 1);
                    j += 1;
                }
            }
        }
        v[n - 1]
    }
}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        fn helper(n: usize) -> Vec<i32> {
            match n {
                0 => vec![0],
                1 => vec![0, 1],
                2 => vec![0, 1, 1],
                _ => {
                    let mut v = helper(n - 1);
                    let t_n = v[n - 3] + v[n - 2] + v[n - 1];
                    v.push(t_n);
                    v
                },
            }
        }
        let n = n as usize;
        helper(n)[n]
    }
}

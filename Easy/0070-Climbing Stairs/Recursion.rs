impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fn helper(n: usize) -> Vec<i32> {
            match n {
                1 => vec![1],
                2 => vec![1, 2],
                _ => {
                    let mut v = helper(n - 1);
                    v.push(v[n - 2] + v[n - 3]);
                    v
                },
            }
        }
        helper(n as usize).pop().unwrap()
    }
}

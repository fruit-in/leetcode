use std::collections::HashSet;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let pt_sq: Vec<i32> = (1..=n).map(|x| x * x).filter(|&x| x <= n).collect();
        let mut set = HashSet::new();
        let mut v = vec![n];
        let mut cnt = 1;
 
        while !v.is_empty() {
            for _ in 0..v.len() {
                let num = v.remove(0);
                for i in &pt_sq {
                    if num - i == 0 {
                        return cnt;
                    } else if num - i > 0 && !set.contains(&(num - i)) {
                        set.insert(num - i);
                        v.push(num - i);
                    } else if num - i < 0 {
                        break;
                    }
                }
            }
            cnt += 1;
        }
        0
    }
}

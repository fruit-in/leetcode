impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut count = vec![0; k as usize];

        for &x in &arr {
            count[x.rem_euclid(k) as usize] += 1;
        }

        for i in 1..(k as usize + 1) / 2 {
            if count[i] != count[k as usize - i] {
                return false;
            }
        }

        k % 2 == 1 || count[k as usize / 2] % 2 == 0
    }
}

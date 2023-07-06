impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut s = s.into_bytes();
        let mut prefix_sum = vec![0; s.len()];

        for i in 0..shifts.len() {
            if shifts[i][0] > 0 {
                prefix_sum[shifts[i][0] as usize - 1] -= 2 * shifts[i][2] - 1;
            }
            prefix_sum[shifts[i][1] as usize] += 2 * shifts[i][2] - 1;
        }

        for i in (0..prefix_sum.len()).rev() {
            prefix_sum[i] += *prefix_sum.get(i + 1).unwrap_or(&0);
            s[i] = ((s[i] - b'a') as i32 + prefix_sum[i]).rem_euclid(26) as u8 + b'a';
        }

        String::from_utf8(s).unwrap()
    }
}

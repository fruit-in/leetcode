impl Solution {
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        let mut prefix_l = vec![0; security.len()];
        let mut prefix_r = vec![0; security.len()];

        for i in 1..security.len() {
            if security[i] <= security[i - 1] {
                prefix_l[i] = prefix_l[i - 1] + 1;
            }
            if security[security.len() - i - 1] <= security[security.len() - i] {
                prefix_r[security.len() - i - 1] = prefix_r[security.len() - i] + 1;
            }
        }

        (time..security.len() as i32 - time)
            .filter(|&i| prefix_l[i as usize] >= time && prefix_r[i as usize] >= time)
            .collect()
    }
}

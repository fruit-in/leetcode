impl Solution {
    pub fn number_of_ways(s: String) -> i64 {
        let mut count0 = vec![0; s.len()];
        let mut count1 = vec![0; s.len()];

        for (i, building) in s.chars().enumerate() {
            if i > 0 {
                count0[i] = count0[i - 1];
                count1[i] = count1[i - 1];
            }

            count0[i] += (building == '0') as i64;
            count1[i] += (building == '1') as i64;
        }

        s.chars()
            .enumerate()
            .map(|(i, building)| match building {
                '0' => count1[i] * (count1[s.len() - 1] - count1[i]),
                _ => count0[i] * (count0[s.len() - 1] - count0[i]),
            })
            .sum()
    }
}

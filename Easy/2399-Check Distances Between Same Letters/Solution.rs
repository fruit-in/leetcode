impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut distance_s = vec![-1; 26];

        for (i, c) in s.bytes().enumerate() {
            if distance_s[(c - b'a') as usize] == -1 {
                distance_s[(c - b'a') as usize] = i as i32 + 1;
            } else {
                distance_s[(c - b'a') as usize] = i as i32 - distance_s[(c - b'a') as usize];
            }
        }

        distance_s
            .iter()
            .zip(distance.iter())
            .all(|(a, b)| a == b || *a == -1)
    }
}

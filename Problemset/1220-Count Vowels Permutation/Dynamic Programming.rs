impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut end = vec![1 as u32; 5];

        for _ in 1..n {
            let mut tmp = Vec::new();
            tmp.push((end[1] + end[2] + end[4]) % 1_000_000_007);
            tmp.push((end[0] + end[2]) % 1_000_000_007);
            tmp.push((end[1] + end[3]) % 1_000_000_007);
            tmp.push((end[2]) % 1_000_000_007);
            tmp.push((end[2] + end[3]) % 1_000_000_007);
            end = tmp;
        }

        (end.iter().sum::<u32>() % 1_000_000_007) as i32
    }
}

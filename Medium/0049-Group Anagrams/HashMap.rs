use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams = HashMap::new();

        for s in strs {
            let mut cnt = [0; 26];
            s.bytes().for_each(|c| cnt[(c - b'a') as usize] += 1);
            anagrams.entry(cnt).or_insert(Vec::new()).push(s);
        }

        anagrams.values().cloned().collect()
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adjacent = HashMap::new();
        let mut ret = vec![];

        for pair in &adjacent_pairs {
            adjacent.entry(pair[0]).or_insert(vec![]).push(pair[1]);
            adjacent.entry(pair[1]).or_insert(vec![]).push(pair[0]);
        }

        let mut curr = *adjacent.iter().find(|(_, v)| v.len() == 1).unwrap().0;
        ret.push(curr);

        for i in 0..adjacent_pairs.len() {
            curr = match &adjacent.get(&curr).unwrap()[..] {
                &[x] => x,
                &[x, y] if x != ret[i - 1] => x,
                &[x, y] => y,
                _ => 0,
            };
            ret.push(curr);
        }

        ret
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut group_ids = HashMap::new();
        let mut groups = Vec::new();

        for (id, size) in group_sizes.iter().enumerate() {
            let v = group_ids.entry(size).or_insert(Vec::new());
            (*v).push(id as i32);

            if v.len() == *size as usize {
                groups.push(group_ids.remove(size).unwrap());
            }
        }

        groups
    }
}

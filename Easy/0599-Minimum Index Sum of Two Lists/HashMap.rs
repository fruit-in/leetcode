use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut rest_idx = list1
            .iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect::<HashMap<&String, usize>>();
        let mut min_sum = std::usize::MAX;
        let mut ret = Vec::new();

        for i in 0..list2.len() {
            if let Some(j) = rest_idx.get(&list2[i]) {
                if i + j == min_sum {
                    ret.push(list2[i].to_string());
                } else if i + j < min_sum {
                    ret.clear();
                    ret.push(list2[i].to_string());
                    min_sum = i + j;
                }
            }
        }

        ret
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut items = HashMap::new();

        for item in items1.iter().chain(items2.iter()) {
            items
                .entry(item[0])
                .and_modify(|w| *w += item[1])
                .or_insert(item[1]);
        }

        let mut ret = items
            .into_iter()
            .map(|(v, w)| vec![v, w])
            .collect::<Vec<_>>();
        ret.sort_unstable();

        ret
    }
}

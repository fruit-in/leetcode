use std::collections::HashMap;

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, mut pieces: Vec<Vec<i32>>) -> bool {
        let indices = arr
            .iter()
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect::<HashMap<_, _>>();

        pieces.sort_unstable_by_key(|piece| indices.get(&piece[0]));

        arr == pieces.concat()
    }
}

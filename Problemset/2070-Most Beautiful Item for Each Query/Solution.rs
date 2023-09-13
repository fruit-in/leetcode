impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut items = items;
        let mut answer = vec![0; queries.len()];

        items.sort_unstable_by_key(|item| (item[0], -item[1]));

        for i in 1..items.len() {
            items[i][1] = items[i][1].max(items[i - 1][1]);
        }

        for i in 0..queries.len() {
            let j = items
                .binary_search(&vec![queries[i], i32::MAX])
                .unwrap_err();

            answer[i] = match j {
                0 => 0,
                _ => items[j - 1][1],
            };
        }

        answer
    }
}

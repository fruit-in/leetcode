use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut queries = (0..queries.len())
            .map(|i| (queries[i], i))
            .collect::<Vec<_>>();
        let mut i = 0;
        let mut heap = BinaryHeap::new();
        let mut ret = vec![-1; queries.len()];

        intervals.sort_unstable();
        queries.sort_unstable();

        for &(query, j) in &queries {
            while i < intervals.len() && intervals[i][0] <= query {
                heap.push(Reverse((
                    intervals[i][1] - intervals[i][0] + 1,
                    intervals[i][1],
                )));
                i += 1;
            }

            while let Some(&Reverse((size, right))) = heap.peek() {
                if right < query {
                    heap.pop();
                } else {
                    ret[j] = size;
                    break;
                }
            }
        }

        ret
    }
}
